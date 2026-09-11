#!/usr/bin/env python3
"""Replay the framework's validation fixtures in a disposable copy.

Run: python3 governance/validation/replay_checks.py [--write-results]
Every case states its expected exit status; negative cases are intentional.
Fixtures are synthetic and create no actual authority or adoption.
"""
import hashlib
import json
from pathlib import Path
import re
import shutil
import subprocess
import sys
import tempfile
import uuid

PACKAGE = Path(__file__).resolve().parents[2]
# Historical identifiers require retention or an explicit retirement disposition.
IDS_1_0_0 = 'C0:1,2,3,4,5,6,7,8,9,10,11 C1:1,2,3,4,5,6 C2:1,2,3,4,5,6,7,8,9,10,11,12 C3:1,2,3,4,5,6,7 C4:1,2,3,4,5 C5:1,2,3,4,5,6,7,8,9,10,11 C6:1,2,3,4,5,6,7,8,9,10,11 C7:1,2,3,4,5,6 C10:1,2,3 C11:1,2,3,4,5 C8:1,2,3,4,5 C9:1,2,3,4,5,6,7,8,9 I1:1,2 I2:1,2,3,4,5,6,7,8,9,10 I3:1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16 I4:1,2,3,4,5,6,7 I5:1,2,3,4 I6:1,2,3,4,5,6,7,8,9 I7:1,2,3,4,5 I8:2,3,4,5,6,7 I9:1,2,3,4,5,6 I10:1,2,3,4 I11:1,2,4,5,6,7,8,9,10,11 I12:1,2,3,4,5,6,7,8 I13:1,2 I14:1,2,3,4,5,6,7,9,10,11,12,13 I15:1,2,3 I16:1,2,3,4,5,6,7,8,9,10,11,12,13,14 O0:1,2,3,4 OP:2,3,4 OB:1,2,3,4,5 OR:1,2,3,4,5,6 D:1,2,3,4,5,6,7,8,9,10,11 T:1,3 G:1,2,3,4,5,6,7,8 A1:1,2,3,4,5 A2:1,2 A3:1,2,3'
MEMBER_FILES = ('CONSTITUTION.md', 'AGENT_RULES.md', 'ROLE_OVERLAYS.md', 'DESIGN_AUTHORITY_INDEX_SCHEMA.md',
                'TECHNICAL_DECISION_RECORD_TEMPLATE.md', 'GOVERNANCE_TELEMETRY_SCHEMA.md', 'ACTIVATION_MANIFEST.md')
results = []


def sha(data):
    return hashlib.sha256(data).hexdigest()


def clause_ids(root):
    found = set()
    for name in MEMBER_FILES:
        text = (root / 'governance' / name).read_text(encoding='utf-8')
        found.update(re.findall(r'^\*\*([A-Z]+[0-9]*\.[0-9]+)\*\*', text, re.M))
    return found


def old_ids():
    ids = set()
    for group in IDS_1_0_0.split():
        prefix, numbers = group.split(':')
        ids.update(f'{prefix}.{n}' for n in numbers.split(','))
    return ids


with tempfile.TemporaryDirectory(prefix='framework-v112-') as temp:
    root = Path(temp) / 'project'
    shutil.copytree(PACKAGE, root)
    tool = root / 'governance/tools/framework.py'
    manifest = root / 'governance/ACTIVATION_MANIFEST.md'
    agents = root / 'AGENTS.md'
    claude = root / 'CLAUDE.md'

    def run(label, args, code, contains=None):
        r = subprocess.run([sys.executable, str(tool), *args], cwd='/', capture_output=True, text=True)
        assert r.returncode == code, (label, r.stdout, r.stderr)
        if contains:
            assert contains in r.stdout + r.stderr, (label, r.stdout, r.stderr)
        results.append({'case': label, 'expected_exit': code, 'actual_exit': r.returncode,
                        'stdout': r.stdout, 'stderr': r.stderr})
        return r

    def cfg_update(**changes):
        text = manifest.read_text()
        cfg = json.loads(re.search(r'```json\n(.*?)\n```', text, re.S).group(1))
        if 'first_mission_id' in changes:
            writej(root / 'project_records/missions/READINESS.json', {'first_mission_id': changes.pop('first_mission_id')})
        for key in ('per_request_seconds', 'stage_seconds', 'replacement_limit'):
            changes.pop(key, None)
        cfg.update(changes)
        manifest.write_text(re.sub(r'```json\n.*?\n```', lambda _: '```json\n' + json.dumps(cfg, indent=2) + '\n```',
                                   text, flags=re.S))

    def writej(path, obj):
        if path.name == 'RESOURCE_POLICY_REFERENCES.json':
            obj.setdefault('review_limits', {'per_request_seconds': 3600, 'stage_seconds': 14400,
                                            'replacement_limit': 1, 'source': 'Synthetic fixture only'})
        path.write_text(json.dumps(obj, indent=2) + '\n')

    def hashes():
        lines = (root / 'governance/OPERATIVE_SET.sha256').read_text().splitlines()
        return dict((n, h) for h, n in (line.split(maxsplit=1) for line in lines))

    inventory = json.loads((root / 'governance/validation/CLAUSE_INVENTORY.json').read_text())
    assert set(inventory['current']) == clause_ids(root), 'Current clause inventory mismatch'
    retired = inventory['retired']
    assert not (set(retired) & clause_ids(root)), 'Retired clause identifier reused'
    assert all(isinstance(v, str) and v.strip() for v in retired.values()), 'Retirement requires disposition'
    assert old_ids() <= clause_ids(root) | set(retired), 'Historical identifier lacks disposition'
    cfg_update(project_id='UNCONFIGURED', operator_authority='UNCONFIGURED', operator_source='UNCONFIGURED',
               scope='UNCONFIGURED', dependency_assurance='UNCONFIGURED',
               standing_design_discretion='UNCONFIGURED', discretion_source=None)
    writej(root / 'project_records/missions/READINESS.json', {'first_mission_id': None})
    writej(root / 'project_records/governance/RESOURCE_POLICY_REFERENCES.json', {'channels': []})
    row_file = root / 'project_records/decisions/dai/DAI-ENVELOPE-01.json'
    writej(row_file, {'id': 'DAI-ENVELOPE-01', 'kind': 'PROPERTY', 'property': 'Fixture discretion',
                      'status': 'PROPOSED', 'scope': 'Fixture', 'source': None})
    run('Reset synthetic candidate hashes', ['write-hashes'], 0)


    # Structure and startup
    run('Template structure from unrelated cwd', ['check'], 0, 'UNADOPTED')
    run('Template not startup-ready', ['ready'], 1, 'Unconfigured binding')
    cfg_update(project_id='synthetic-fixture', operator_authority='Test fixture',
               operator_source='Synthetic test data, not acceptance', scope='Disposable validation only')
    run('Candidate hash refresh before adoption', ['write-hashes'], 0)
    run('Three bindings alone insufficient', ['ready'], 1, 'dependency_assurance')
    cfg_update(dependency_assurance='SOME', per_request_seconds=3600, stage_seconds=14400, replacement_limit=1,
               standing_design_discretion='DECLINE', discretion_source='Synthetic discretion decision')
    run('Candidate with invalid dependency scope', ['write-hashes'], 0)
    run('Invalid dependency scope rejected', ['ready'], 1, 'ALL, MATERIAL')
    cfg_update(dependency_assurance='NOT_CHEAPLY_REVERSIBLE')
    run('Candidate with removed dependency option', ['write-hashes'], 0)
    run('Removed middle dependency option rejected', ['ready'], 1, 'ALL, MATERIAL')
    cfg_update(dependency_assurance='MATERIAL')
    run('Explicit manual-channel timing candidate', ['write-hashes'], 0)
    run('No channel not ready', ['ready'], 1, 'No recorded authorized')
    resource = root / 'project_records/governance/RESOURCE_POLICY_REFERENCES.json'
    writej(resource, {'channels': [{'id': 'fixture', 'source': None, 'available': True, 'fresh_instances': 2}]})
    run('Channel without source refused', ['ready'], 1, 'authority source')
    writej(resource, {'channels': [{'id': 'fixture', 'source': 'Synthetic authorization reference',
                                    'available': True, 'fresh_instances': 2}]})
    run('No first grant not ready', ['ready'], 1, 'first_mission_id')
    mission = run('Mission scaffold creates no grant', ['new-mission', '--title', 'Fixture mission'], 0,
                  'No authority granted').stdout.splitlines()[0]
    folder = root / 'project_records/missions' / mission
    assert (folder / 'checkpoint.md').is_file() and (folder / 'closeout.md').is_file()
    cfg_update(first_mission_id=mission)
    run('Refresh configured candidate hashes', ['write-hashes'], 0)
    run('Empty grant refused', ['ready'], 1, 'grant_source')
    mission_file = folder / 'mission.json'
    record = json.loads(mission_file.read_text())
    record.update(grant_source='Synthetic grant source', objective='Fixture', scope='Fixture only',
                  duty_holder='Fixture owner')
    writej(mission_file, record)
    run('Grant without actors, duration and conditions refused', ['ready'], 1, 'First mission missing: actors')
    record.update(actors=['Fixture owner'], duration='Until fixture closeout', conditions='Synthetic only')
    writej(mission_file, record)
    run('Discretion decision must match DAI', ['ready'], 1, 'DAI setup row')
    row_file = root / 'project_records/decisions/dai/DAI-ENVELOPE-01.json'
    row = json.loads(row_file.read_text())
    row.update(source='Synthetic discretion decision', setup_disposition='DECLINE')
    writej(row_file, row)
    run('Configured startup records pass', ['ready'], 0, 'READY RECORDS PASS')
    # S.P.A.R.K. migration and operational-policy regression checks.
    original_mid = mission
    original_folder = folder
    legacy_id = 'PHASE2-GATE-C2'
    legacy_folder = folder.parent / legacy_id
    folder.rename(legacy_folder)
    record['id'] = legacy_id
    writej(legacy_folder / 'mission.json', record)
    cfg_update(first_mission_id=legacy_id)
    run('Existing mission ID accepted without renumbering', ['ready'], 0, 'READY RECORDS PASS')
    run('Existing mission ID appears in index', ['index'], 0, legacy_id)
    cfg_update(first_mission_id='../outside')
    run('Mission pointer traversal refused', ['ready'], 1, 'first_mission_id')
    cfg_update(first_mission_id=original_mid)
    legacy_folder.rename(original_folder)
    record['id'] = original_mid
    writej(original_folder / 'mission.json', record)
    policy = json.loads(resource.read_text())
    policy['review_limits']['per_request_seconds'] = 0
    resource.write_text(json.dumps(policy))
    run('Nonpositive operational deadline refused', ['ready'], 1, 'positive review limit')
    policy['review_limits']['per_request_seconds'] = 3600
    policy['review_limits']['stage_seconds'] = 1
    resource.write_text(json.dumps(policy))
    run('Operational stage shorter than request refused', ['ready'], 1, 'shorter than request')
    policy['review_limits']['stage_seconds'] = 14400
    resource.write_text(json.dumps(policy))
    proposed = dict(row, setup_disposition='PROPOSED_ADOPT')
    writej(row_file, proposed)
    run('Proposed discretion cannot satisfy readiness', ['ready'], 1, 'Declined discretion')
    writej(row_file, row)
    second = run('Second creator gets distinct mission ID', ['new-mission'], 0).stdout.splitlines()[0]
    assert second != mission

    # Adoption fidelity
    accepted_hashes = hashes()
    rec = root / 'project_records/governance/adoptions/fixture.json'
    acceptance = {'status': 'ADOPTED', 'project_id': 'synthetic-fixture', 'framework_version': '1.1.3',
                  'operator_source': 'Synthetic test data only; not a real adoption',
                  'accepted_at': '2026-09-11T00:00:00Z', 'prior_governance_disposition': 'Synthetic fixture: none',
                  'verification_evidence': 'This disposable test', 'member_hashes': accepted_hashes,
                  'supersedes': None}
    writej(rec, acceptance)
    accepted_digest = sha(rec.read_bytes())
    pointer = root / 'project_records/governance/ACTIVE_ADOPTION.json'
    writej(pointer, {'record': rec.relative_to(root).as_posix(), 'sha256': accepted_digest})
    run('Separate accepted identity matches', ['check', '--expected-adoption-sha256', accepted_digest], 0,
        'ACCEPTED BY REFERENCED RECORD')
    policy = json.loads(resource.read_text())
    policy['review_limits']['per_request_seconds'] = 7200
    policy['review_limits']['stage_seconds'] = 28800
    resource.write_text(json.dumps(policy))
    run('Operational timing change preserves accepted governing identity',
        ['check', '--expected-adoption-sha256', accepted_digest], 0, 'ACCEPTED BY REFERENCED RECORD')
    run('Changed operational timing remains ready', ['ready'], 0, 'READY RECORDS PASS')
    rules = root / 'governance/AGENT_RULES.md'
    clean = rules.read_bytes()
    original_list = (root / 'governance/OPERATIVE_SET.sha256').read_bytes()
    rules.write_bytes(clean + b'\nAgents may skip assurance when busy.\n')
    run('Fake governing rule rejected', ['check'], 1, 'differ from accepted content')
    run('Hash writer refuses adopted state', ['write-hashes'], 1, 'regeneration refused')
    current = {name: sha((root / name).read_bytes()) for name in accepted_hashes}
    (root / 'governance/OPERATIVE_SET.sha256').write_text(''.join(current[n] + '  ' + n + '\n' for n in accepted_hashes))
    run('Manual member-list regeneration cannot bless rule', ['check'], 1, 'differ from accepted content')
    candidate = root / 'candidate.sha256'
    before = rec.read_bytes(), pointer.read_bytes(), (root / 'governance/OPERATIVE_SET.sha256').read_bytes()
    run('Separate amendment candidate list allowed', ['candidate-hashes', '--output', str(candidate)], 0, 'Separate candidate')
    assert before == (rec.read_bytes(), pointer.read_bytes(), (root / 'governance/OPERATIVE_SET.sha256').read_bytes())
    run('Candidate output cannot replace active list',
        ['candidate-hashes', '--output', str(root / 'governance/OPERATIVE_SET.sha256')], 1, 'cannot replace')
    rules.write_bytes(clean)
    (root / 'governance/OPERATIVE_SET.sha256').write_bytes(original_list)
    rules.write_bytes(clean.replace(b'\n', b'\r\n'))
    run('CRLF drift reported without regeneration', ['check'], 1, 'Expected LF')
    rules.write_bytes(clean)
    run('Restored accepted LF bytes pass', ['check'], 0)
    original_record = rec.read_bytes()
    acceptance['operator_source'] = 'Changed synthetic source'
    writej(rec, acceptance)
    run('Acceptance record drift rejected', ['check'], 1, 'record digest mismatch')
    writej(pointer, {'record': rec.relative_to(root).as_posix(), 'sha256': sha(rec.read_bytes())})
    run('Trusted digest detects rewritten local record and pointer',
        ['check', '--expected-adoption-sha256', accepted_digest], 1, 'independently supplied digest')
    rec.write_bytes(original_record)
    writej(pointer, {'record': None, 'sha256': None})
    run('Removing pointer leaves detectable acceptance history', ['check'], 1, 'no active pointer')
    run('History blocks hash regeneration', ['write-hashes'], 1, 'regeneration refused')
    writej(pointer, {'record': rec.relative_to(root).as_posix(), 'sha256': accepted_digest})

    # Entrypoints (A6)
    (root / 'PROJECT_CONVENTIONS.md').write_text('Use project command: example-test.\n')
    agents_clean = agents.read_text()
    with agents.open('a') as f:
        f.write('\nOrdinary example convention.\n')
    run('Ordinary conventions do not amend governing identity', ['check'], 0)
    agents.write_text('# Project agent entrypoint\n\nUse pnpm.\n')
    run('Entrypoint block removed from AGENTS.md rejected', ['check'], 1, 'A6 entrypoint block')
    agents.write_text(agents_clean.replace('never repair drift', 'may repair drift'))
    run('Altered entrypoint block rejected', ['check'], 1, 'A6 entrypoint block')
    agents.unlink()
    run('Missing AGENTS.md rejected', ['check'], 1, 'AGENTS.md is missing')
    agents.write_text(agents_clean + '\n' + ('Example convention line for budget testing.\n' * 600))
    run('Oversized AGENTS.md warns without failing', ['check'], 0, 'Codex instruction budget')
    agents.write_text(agents_clean)
    claude_clean = claude.read_text()
    claude.write_text(claude_clean.replace('@governance/AGENT_RULES.md\n', ''))
    run('CLAUDE.md missing required import rejected', ['check'], 1, 'does not import governance/AGENT_RULES.md')
    claude.write_text(claude_clean + '@docs/MISSING.md\n')
    run('CLAUDE.md unresolved import rejected', ['check'], 1, 'imports a missing file')
    claude.unlink()
    run('Absent CLAUDE.md allowed', ['check'], 0)
    claude.write_text(claude_clean)

    # Readable index and DAI row validation
    run('Readable index lists rows and missions', ['index'], 0, 'Fixture mission')
    dai = root / 'project_records/decisions/dai'
    bad_id = 'DAI-' + str(uuid.uuid4())
    base_row = {'id': bad_id, 'kind': 'PROPERTY', 'property': 'Fixture retention period', 'status': 'MAYBE',
                'scope': 'Fixture', 'source': None}
    writej(dai / (bad_id + '.json'), base_row)
    run('Invalid DAI status rejected', ['index'], 1, 'status must be one of')
    writej(dai / (bad_id + '.json'), dict(base_row, status='SETTLED'))
    run('SETTLED row without source rejected', ['index'], 1, 'requires its authoritative source')
    writej(dai / (bad_id + '.json'), dict(base_row, status='PROVISIONAL-AGENT'))
    run('Provisional row without containment fields rejected', ['index'], 1, 'provisional row missing')
    writej(dai / (bad_id + '.json'), dict(base_row, status='PROVISIONAL-AGENT', containment='Export module only',
                                          reversibility='CHEAP', reassessment_date='2026-10-01',
                                          review_priority='ROUTINE', hardening_condition='Operator sets retention'))
    run('Complete provisional row enters review queue', ['index'], 0, 'Operator sets retention')
    writej(dai / 'DAI-renamed.json', dict(base_row, status='PROPOSED'))
    run('DAI id/filename mismatch rejected', ['index'], 1, 'id does not match filename')
    (dai / 'DAI-renamed.json').unlink()

    # Telemetry
    log = folder / 'governance_events.jsonl'
    event = {'schema_version': '1.0', 'event_id': 'EVT-' + str(uuid.uuid4()), 'issue_id': 'ISS-' + str(uuid.uuid4()),
             'mission_id': mission, 'actor': 'fixture-owner', 'event_time': '2026-09-11T00:00:00Z',
             'types': ['HOLD'], 'trigger': {'description': 'Synthetic missing review', 'clause': 'I11.7'},
             'affected_work': ['fixture'], 'evidence': ['evidence/synthetic.txt']}

    def events(items):
        log.write_text(''.join(json.dumps(x) + '\n' for x in items))

    events([event])
    run('Concrete JSON event validates', ['check-events'], 0, '1 events')
    update = dict(event, event_id='EVT-' + str(uuid.uuid4()), event_time='2026-09-11T00:01:00Z')
    events([event, update])
    run('Distinct updates retain common issue', ['check-events'], 0, '2 events')
    events([event, event])
    run('Duplicate event identities rejected', ['check-events'], 1, 'Duplicate event')
    events([{'Issue ID': 'example'}])
    run('Human labels are not JSON schema keys', ['check-events'], 1, 'missing required')
    failure = dict(event, types=['FAILURE'])
    events([failure])
    run('Failure requires failure facts', ['check-events'], 1, 'failure fields')
    failure.update(failure_classes=['PROGRESS'], failure_status='SUSPECTED', failure_action='Fixture wait',
                   failure_consequence='Fixture delay')
    events([failure])
    run('Complete failure event accepted', ['check-events'], 0)
    events([dict(event, mission_id=second)])
    run('Cross-mission event rejected', ['check-events'], 1, 'another mission')
    events([dict(event, event_time='2026-09-11T00:00:00')])
    run('Timezone-free event rejected', ['check-events'], 1, 'UTC offset')
    events([dict(event, boundary_reached=True, branch='PROJECT_PROPERTY', branch_reason='MISSING_AUTHORITY')])
    run('Branch reason for missing authority accepted (C11.2)', ['check-events'], 0)
    events([dict(event, boundary_reached=True, branch='RESERVED_AUTHORITY', branch_reason='UNRESOLVED_INTERPRETATION')])
    run('Branch reason for unresolved interpretation accepted (C11.2)', ['check-events'], 0)
    events([dict(event, boundary_reached=True, branch_reason='COMPETING_COURSES')])
    run('Retired competing-courses branch reason rejected', ['check-events'], 1, 'invalid enum value')
    legacy_log_folder = folder.parent / 'PHASE2-GATE-C2'
    legacy_log_folder.mkdir()
    legacy_event = dict(event, mission_id='PHASE2-GATE-C2', event_id='EVT-' + str(uuid.uuid4()))
    (legacy_log_folder / 'governance_events.jsonl').write_text(json.dumps(legacy_event) + '\n')
    events([event])
    run('Existing mission ID telemetry validates', ['check-events'], 0, '2 events')

summary = {
    'case_count': len(results),
    'all_expected_results_observed': True,
    'historical_ids_accounted_for': len(old_ids()),
    'current_ids': len(clause_ids(PACKAGE)),
    'root_agents_bytes': (PACKAGE / 'AGENTS.md').stat().st_size,
    'default_codex_budget_bytes': 32768,
    'live_agent_loading': 'not tested',
    'actual_project_adoption': 'not performed',
    'test_scope': 'disposable structural, configuration, entrypoint, record and accepted-content fixtures; '
                  'no human authority authenticated',
}
if '--write-results' in sys.argv:
    out = PACKAGE / 'governance/validation/RESULTS.json'
    out.write_text(json.dumps(dict(summary, cases=results), indent=2) + '\n', encoding='utf-8')
print(json.dumps(summary))
