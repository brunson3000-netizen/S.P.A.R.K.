#!/usr/bin/env python3
"""Local structural/readiness/accepted-content checks; never grants or adopts authority."""
import argparse
from datetime import datetime
import hashlib
import json
from pathlib import Path
import re
import sys
import uuid

MEMBERS = tuple('governance/' + name for name in (
    'CONSTITUTION.md', 'AGENT_RULES.md', 'ROLE_OVERLAYS.md',
    'DESIGN_AUTHORITY_INDEX_SCHEMA.md', 'TECHNICAL_DECISION_RECORD_TEMPLATE.md',
    'GOVERNANCE_TELEMETRY_SCHEMA.md', 'ACTIVATION_MANIFEST.md'))
HASHES = 'governance/OPERATIVE_SET.sha256'
POINTER = 'project_records/governance/ACTIVE_ADOPTION.json'
ADOPTIONS = 'project_records/governance/adoptions'
DAI_DIR = 'project_records/decisions/dai'
MISSIONS = 'project_records/missions'
UUID4 = r'[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}'
DEPENDENCY_SCOPES = ('ALL', 'MATERIAL')
GRANT_FIELDS = ('title', 'grant_source', 'objective', 'scope', 'actors', 'duty_holder', 'duration', 'conditions')
DAI_KINDS = ('PROPERTY', 'TECHNOLOGY_DIRECTION')
DAI_STATUSES = ('SETTLED', 'RESERVED', 'DEFERRED', 'PROVISIONAL-AGENT', 'PROPOSED', 'SUPERSEDED')
SOURCED_STATUSES = ('SETTLED', 'RESERVED', 'DEFERRED', 'SUPERSEDED')
PROVISIONAL_FIELDS = ('containment', 'reversibility', 'reassessment_date', 'review_priority', 'hardening_condition')
CODEX_BUDGET = 32 * 1024  # Codex default project_doc_max_bytes, shared across discovered AGENTS.md files
CODEX_WARN_AT = 0.75
WARNINGS = []


def require(condition, message):
    if not condition:
        raise ValueError(message)


def content(value):
    return isinstance(value, str) and bool(value.strip()) and value.strip() != 'UNCONFIGURED'


def filled(value):
    if isinstance(value, list):
        return bool(value) and all(content(item) for item in value)
    return content(value)


def digest(data):
    return hashlib.sha256(data).hexdigest()


def location(root, relative):
    p = Path(relative)
    require(not p.is_absolute() and '..' not in p.parts, 'Record path must stay inside the project.')
    path = (root / p).resolve()
    require(path.is_relative_to(root), 'Record resolves outside the project.')
    return path


def read_json(path):
    return json.loads(path.read_text(encoding='utf-8'))


def json_block(path):
    blocks = re.findall(r'```json\s*\n(.*?)\n```', path.read_text(encoding='utf-8'), re.S)
    require(len(blocks) == 1, f'Expected one canonical JSON block: {path.name}')
    return json.loads(blocks[0])


def manifest_section(root, heading):
    text = (root / MEMBERS[-1]).read_text(encoding='utf-8')
    # Split only at manifest section headings; the A6 block contains its own heading.
    parts = re.split(r'^## (?=A[0-9]+ |Configuration$)', text, flags=re.M)
    matches = [part for part in parts if part.startswith(heading + ' ')]
    require(len(matches) == 1, f'Manifest section missing: {heading}')
    return matches[0]


def identities(root):
    return {name: digest(location(root, name).read_bytes()) for name in MEMBERS}


def hash_text(values):
    return ''.join(values[name] + '  ' + name + '\n' for name in MEMBERS)


def structure(root):
    ids = set()
    for name in MEMBERS:
        path = location(root, name)
        data = path.read_bytes()
        require(b'\r' not in data, f'Expected LF bytes: {name}')
        text = data.decode('utf-8')
        for clause in re.findall(r'^\*\*([A-Z]+[0-9]*\.[0-9]+)\*\*', text, re.M):
            require(clause not in ids, f'Duplicate clause: {clause}')
            ids.add(clause)
        for target in re.findall(r'\]\(([^)]+)\)', text):
            if '://' in target or target.startswith('#'):
                continue
            require((path.parent / target.split('#', 1)[0]).is_file(), f'Missing member link: {name}: {target}')
    manifest = (root / MEMBERS[-1]).read_text(encoding='utf-8')
    inventory = re.findall(r'^\| [1-7] — .*?\]\(([^)]+)\)', manifest, re.M)
    require(tuple('governance/' + n for n in inventory) == MEMBERS, 'Manifest member inventory mismatch.')
    for name in (POINTER, 'project_records/governance/RESOURCE_POLICY_REFERENCES.json',
                 'project_records/decisions/PROJECT_SOURCE_REGISTER.md',
                 'project_records/decisions/DESIGN_AUTHORITY_INDEX.md',
                 DAI_DIR + '/DAI-ENVELOPE-01.json',
                 MISSIONS + '/MISSION_REGISTRY.md', MISSIONS + '/READINESS.json'):
        require(location(root, name).is_file(), f'Missing record-home entry: {name}')
    for name in (ADOPTIONS, 'project_records/decisions/tdr', DAI_DIR, 'project_records/decisions/operator_input'):
        require(location(root, name).is_dir(), f'Missing record directory: {name}')
    return ids


def entrypoints(root):
    """A6: the fixed AGENTS.md block and the required CLAUDE.md imports."""
    a6 = manifest_section(root, 'A6')
    blocks = re.findall(r'```text\n(.*?)\n```', a6, re.S)
    require(len(blocks) == 1, 'Manifest A6 must contain one entrypoint block.')
    agents = root / 'AGENTS.md'
    require(agents.is_file(), 'Root AGENTS.md is missing; restore it with the A6 entrypoint block (C9.10).')
    data = agents.read_bytes()
    require(blocks[0] in data.decode('utf-8').replace('\r\n', '\n'),
            'Root AGENTS.md lacks the exact A6 entrypoint block; restore it from the manifest (C9.10).')
    if len(data) > CODEX_BUDGET * CODEX_WARN_AT:
        WARNINGS.append(f'Root AGENTS.md is {len(data)} bytes, {len(data) * 100 // CODEX_BUDGET}% of the default '
                        '32 KiB Codex instruction budget; move conventions to PROJECT_CONVENTIONS.md.')
    claude = root / 'CLAUDE.md'
    if claude.is_file():
        imports = re.findall(r'^@(\S+)\s*$', claude.read_text(encoding='utf-8'), re.M)
        clause = re.search(r'^\*\*A6\.2\*\*(.*)$', a6, re.M)
        require(clause is not None, 'Manifest A6.2 is missing.')
        required = [path for path in re.findall(r'`([^`]+)`', clause.group(1)) if path != 'CLAUDE.md']
        for path in required:
            require(path in imports, f'CLAUDE.md does not import {path} (A6.2).')
        for path in imports:
            require(location(root, path).is_file(), f'CLAUDE.md imports a missing file: {path} (A6.2).')


def configuration(root):
    cfg = json_block(root / MEMBERS[-1])
    require(isinstance(cfg, dict), 'Manifest configuration must be an object.')
    for key in ('project_id', 'operator_authority', 'operator_source', 'scope'):
        require(content(cfg.get(key)), f'Unconfigured binding: {key}')
    require(cfg.get('dependency_assurance') in DEPENDENCY_SCOPES,
            'Select dependency_assurance: ' + ', '.join(DEPENDENCY_SCOPES) + '.')
    require(cfg.get('standing_design_discretion') in ('ADOPT', 'NARROW', 'DECLINE'), 'Select ADOPT, NARROW or DECLINE for discretion.')
    require(content(cfg.get('discretion_source')), 'Record the actual discretion decision source.')
    return cfg


def adoption(root, values, expected=None):
    pointer = read_json(root / POINTER)
    require(isinstance(pointer, dict) and set(pointer) == {'record', 'sha256'}, 'Malformed active-adoption pointer.')
    existing = list((root / ADOPTIONS).glob('*.json'))
    if pointer['record'] is None:
        require(pointer['sha256'] is None, 'Digest without active record.')
        require(not existing, 'Acceptance records exist but no active pointer; reconcile, do not regenerate.')
        require(expected is None, 'Trusted acceptance digest supplied but no active record.')
        return None
    require(content(pointer['record']), 'Invalid acceptance record path.')
    path = location(root, pointer['record'])
    require(path.parent == (root / ADOPTIONS).resolve(), 'Active record must be directly under adoptions/.')
    raw = path.read_bytes()
    require(digest(raw) == pointer['sha256'], 'Acceptance record digest mismatch.')
    if expected is not None:
        require(digest(raw) == expected, 'Acceptance record differs from independently supplied digest.')
    record = json.loads(raw)
    require(isinstance(record, dict), 'Acceptance record must be an object.')
    require(record.get('status') == 'ADOPTED', 'Record is not ADOPTED.')
    cfg = configuration(root)
    require(record.get('project_id') == cfg['project_id'], 'Accepted project binding mismatch.')
    version = re.search(r'framework version ([0-9]+\.[0-9]+\.[0-9]+),', (root / MEMBERS[-1]).read_text(encoding='utf-8'))
    require(version is not None and record.get('framework_version') == version.group(1), 'Accepted framework version mismatch.')
    for key in ('operator_source', 'accepted_at', 'prior_governance_disposition', 'verification_evidence'):
        require(content(record.get(key)), f'Acceptance evidence missing: {key}')
    time_value(record['accepted_at'])
    require(record.get('member_hashes') == values, 'Current member bytes differ from accepted content.')
    return record


def check(root, expected=None):
    ids = structure(root)
    entrypoints(root)
    values = identities(root)
    # Check acceptance independently, including when someone regenerated the member list.
    record = adoption(root, values, expected)
    require((root / HASHES).read_text(encoding='utf-8') == hash_text(values), 'Candidate/member hash list mismatch.')
    state = 'ACCEPTED BY REFERENCED RECORD' if record else 'UNADOPTED TEMPLATE/CANDIDATE'
    print(f'PASS: 7 member identities; {len(ids)} unique clause IDs; entrypoints; {state}.')
    print('Source assertions and human authority are not authenticated by a local structural check.')
    return values


def ready(root):
    check(root)
    cfg = configuration(root)
    pool = read_json(root / 'project_records/governance/RESOURCE_POLICY_REFERENCES.json')
    require(isinstance(pool, dict) and isinstance(pool.get('channels'), list), 'Resource policy needs a channels list.')
    limits = pool.get('review_limits', {})
    require(isinstance(limits, dict), 'Review limits must be an object.')
    for key in ('per_request_seconds', 'stage_seconds'):
        require(type(limits.get(key)) is int and limits[key] > 0, f'Record a positive review limit: {key}')
    require(limits['stage_seconds'] >= limits['per_request_seconds'], 'Stage duration is shorter than request duration.')
    require(type(limits.get('replacement_limit')) is int and limits['replacement_limit'] >= 0, 'Record a nonnegative replacement_limit.')
    require(content(limits.get('source')), 'Review limits lack an actual authority source.')
    eligible = []
    names = set()
    for ch in pool['channels']:
        require(isinstance(ch, dict), 'Invalid channel record.')
        require(content(ch.get('id')) and ch['id'] not in names, 'Missing or duplicate channel id.')
        names.add(ch['id'])
        require(content(ch.get('source')), 'Channel lacks an actual authority source.')
        require(type(ch.get('available')) is bool, 'Channel availability must be boolean.')
        require(type(ch.get('fresh_instances')) is int and ch['fresh_instances'] >= 0, 'Invalid fresh-instance count.')
        if ch['available'] and ch['fresh_instances'] > 0:
            eligible.append(ch)
    require(eligible, 'No recorded authorized, available challenge channel with a fresh instance.')
    mid = read_json(root / MISSIONS / 'READINESS.json').get('first_mission_id')
    require(isinstance(mid, str) and re.fullmatch(r'[A-Za-z0-9][A-Za-z0-9._-]*', mid), 'Record first_mission_id from an actual mission folder.')
    folder = location(root, MISSIONS + '/' + mid)
    mission = read_json(folder / 'mission.json')
    require(isinstance(mission, dict), 'Mission record must be an object.')
    require(mission.get('id') == mid, 'Mission id/path mismatch.')
    for key in GRANT_FIELDS:
        require(filled(mission.get(key)), f'First mission missing: {key}')
    for sub in ('reviews', 'dispensations', 'evidence'):
        require((folder / sub).is_dir(), f'Missing mission directory: {sub}')
    for name in ('governance_events.jsonl', 'checkpoint.md'):
        require((folder / name).is_file(), f'Missing mission record: {name}')
    row = read_json(root / DAI_DIR / 'DAI-ENVELOPE-01.json')
    require(isinstance(row, dict), 'DAI setup row must be an object.')
    require(row.get('source') == cfg['discretion_source'], 'DAI setup row does not cite the discretion decision.')
    choice = cfg['standing_design_discretion']
    if choice == 'DECLINE':
        require(row.get('status') == 'PROPOSED' and row.get('setup_disposition') == 'DECLINE', 'Declined discretion must remain unadopted and recorded as declined.')
    else:
        require(row.get('status') == 'SETTLED' and row.get('setup_disposition') == choice, 'Adopted/narrowed discretion must have a consistent source-established row.')
        require(content(row.get('scope_source')), 'Adopted/narrowed discretion needs its exact scope source.')
    print('READY RECORDS PASS: explicit choices, channel, mission grant and discretion disposition populated.')
    print('No live channel call or grant authentication performed; actual required assurance remains to be completed.')


def time_value(value):
    require(isinstance(value, str), 'Timestamp must be a string.')
    try:
        parsed = datetime.fromisoformat(value.replace('Z', '+00:00'))
    except ValueError:
        raise ValueError('Invalid ISO-8601 timestamp.')
    require(parsed.tzinfo is not None, 'Timestamp needs a UTC offset.')


def schema_check(value, schema, trail='$'):
    if 'enum' in schema:
        require(value in schema['enum'], f'{trail}: invalid enum value.')
    kind = schema.get('type')
    classes = {'string': str, 'object': dict, 'array': list, 'boolean': bool, 'integer': int}
    if kind:
        require(type(value) is classes[kind], f'{trail}: expected {kind}.')
    if kind == 'object':
        require(set(schema.get('required', [])) <= set(value), f'{trail}: missing required keys.')
        props = schema.get('properties', {})
        if schema.get('additionalProperties') is False:
            require(set(value) <= set(props), f'{trail}: unknown keys.')
        for key, child in value.items():
            if key in props:
                schema_check(child, props[key], trail + '.' + key)
    if kind == 'array':
        require(len(value) >= schema.get('minItems', 0), f'{trail}: too few items.')
        if schema.get('uniqueItems'):
            require(len({json.dumps(x, sort_keys=True) for x in value}) == len(value), f'{trail}: repeated items.')
        for i, child in enumerate(value):
            schema_check(child, schema.get('items', {}), trail + '[' + str(i) + ']')
    if kind == 'string':
        require(len(value.strip()) >= schema.get('minLength', 0), f'{trail}: empty string.')
        if 'pattern' in schema:
            require(re.search(schema['pattern'], value) is not None, f'{trail}: invalid identifier.')
        if schema.get('format') == 'date-time':
            time_value(value)


def check_events(root):
    check(root)
    schema = json_block(root / 'governance/GOVERNANCE_TELEMETRY_SCHEMA.md')
    ids = set()
    count = 0
    for log in sorted((root / MISSIONS).glob('*/governance_events.jsonl')):
        for number, line in enumerate(log.read_text(encoding='utf-8').splitlines(), 1):
            require(bool(line.strip()), f'Blank JSONL line: {log}:{number}')
            event = json.loads(line)
            schema_check(event, schema)
            require(event['mission_id'] == log.parent.name, 'Event belongs to another mission.')
            require(event['event_id'] not in ids, 'Duplicate event ID across mission logs.')
            ids.add(event['event_id'])
            if 'FAILURE' in event['types']:
                require({'failure_classes', 'failure_status', 'failure_action', 'failure_consequence'} <= set(event), 'FAILURE event lacks required failure fields.')
            count += 1
    print(f'PASS: {count} events; schema fields, mission identity and unique event IDs checked.')


def cell(value):
    if isinstance(value, list):
        value = ', '.join(str(v) for v in value)
    return str(value if value not in (None, '') else '—').replace('|', '/').replace('\n', ' ')


def index(root):
    """Read-only readable view of DAI rows and missions; validates row shape."""
    check(root)
    rows = []
    for path in sorted((root / DAI_DIR).glob('*.json')):
        row = read_json(path)
        require(isinstance(row, dict), f'{path.name}: row must be an object.')
        require(row.get('id') == path.stem, f'{path.name}: id does not match filename.')
        require(row.get('kind') in DAI_KINDS, f'{path.name}: kind must be one of {", ".join(DAI_KINDS)}.')
        require(row.get('status') in DAI_STATUSES, f'{path.name}: status must be one of {", ".join(DAI_STATUSES)}.')
        require(content(row.get('property')), f'{path.name}: property is required; it serves as the title (A2.5).')
        if row['status'] in SOURCED_STATUSES:
            require(content(row.get('source')), f'{path.name}: {row["status"]} requires its authoritative source.')
        if row['status'] == 'SUPERSEDED':
            require(content(row.get('replacement')), f'{path.name}: SUPERSEDED requires its replacement.')
        if row['status'] == 'PROVISIONAL-AGENT':
            for key in PROVISIONAL_FIELDS:
                require(content(row.get(key)), f'{path.name}: provisional row missing {key}.')
        rows.append(row)
    missions = []
    for folder in sorted(p for p in (root / MISSIONS).iterdir() if p.is_dir() and (p / 'mission.json').is_file()):
        record = read_json(folder / 'mission.json')
        require(isinstance(record, dict) and record.get('id') == folder.name, f'{folder.name}: mission id/path mismatch.')
        if not content(record.get('title')):
            WARNINGS.append(f'{folder.name}: mission has no title (A2.5).')
        missions.append(record)
    print('\n## Design Authority Index\n')
    print('| ID | Property | Kind | Status | Source |')
    print('|---|---|---|---|---|')
    for row in rows:
        print(f"| {row['id']} | {cell(row.get('property'))} | {row['kind']} | {row['status']} | {cell(row.get('source'))} |")
    queue = [row for row in rows if row['status'] == 'PROVISIONAL-AGENT']
    print('\n## Provisional review queue\n')
    if queue:
        print('| ID | Property | Priority | Reassess | Hardening condition |')
        print('|---|---|---|---|---|')
        for row in sorted(queue, key=lambda r: str(r.get('reassessment_date'))):
            print(f"| {row['id']} | {cell(row['property'])} | {cell(row['review_priority'])} | "
                  f"{cell(row['reassessment_date'])} | {cell(row['hardening_condition'])} |")
    else:
        print('None.')
    print('\n## Missions\n')
    print('| ID | Title | Duty holder | Grant recorded |')
    print('|---|---|---|---|')
    for record in missions:
        granted = 'yes' if content(record.get('grant_source')) else 'no'
        print(f"| {record['id']} | {cell(record.get('title'))} | {cell(record.get('duty_holder'))} | {granted} |")
    print('\nThis view is generated from the records; it is a retrieval aid, not a second authority.')


CHECKPOINT = '''# Durable checkpoint

Maintained by the duty holder under I14.10 so a successor can resume without conversation memory; it creates no authority.

| Field | Content |
|---|---|
| Updated (with UTC offset) / by | |
| Repository, workspace and exact baseline | |
| Current state | |
| Completed and verified | |
| Active, waiting and blocked work | |
| Open decisions and holds | |
| Provisional properties (DAI identifiers) | |
| Evidence references | |
| Next action | |
'''


def new_mission(root, title):
    mid = 'MIS-' + str(uuid.uuid4())
    folder = root / MISSIONS / mid
    folder.mkdir(parents=True, exist_ok=False)
    for name in ('reviews', 'dispensations', 'evidence'):
        (folder / name).mkdir()
    record = {'id': mid, 'title': title, 'grant_source': None, 'operator_wording': None,
              'agent_interpretation': None, 'objective': None, 'scope': None, 'exclusions': None,
              'actors': [], 'duty_holder': None, 'role': None, 'duration': None, 'conditions': None,
              'delegated_from': None, 'governance_identity': None}
    (folder / 'mission.json').write_text(json.dumps(record, indent=2) + '\n', encoding='utf-8')
    (folder / 'governance_events.jsonl').write_text('', encoding='utf-8')
    (folder / 'checkpoint.md').write_text(CHECKPOINT, encoding='utf-8')
    (folder / 'closeout.md').write_text('# Mission closeout\n\nNo completed work is claimed by this template.\n', encoding='utf-8')
    print(mid)
    print('Created unconfigured records only; fill the actual grant. No authority granted.')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--root', type=Path, default=Path(__file__).resolve().parents[2])
    subs = parser.add_subparsers(dest='command', required=True)
    c = subs.add_parser('check'); c.add_argument('--expected-adoption-sha256')
    subs.add_parser('ready'); subs.add_parser('write-hashes')
    c = subs.add_parser('candidate-hashes'); c.add_argument('--output', type=Path, required=True)
    c = subs.add_parser('new-mission'); c.add_argument('--title')
    subs.add_parser('check-events'); subs.add_parser('index')
    args = parser.parse_args()
    root = args.root.resolve()
    status = 0
    try:
        if args.command == 'check':
            check(root, args.expected_adoption_sha256)
        elif args.command == 'ready':
            ready(root)
        elif args.command == 'new-mission':
            new_mission(root, args.title)
        elif args.command == 'check-events':
            check_events(root)
        elif args.command == 'index':
            index(root)
        elif args.command == 'write-hashes':
            pointer = read_json(root / POINTER)
            require(pointer == {'record': None, 'sha256': None} and not list((root / ADOPTIONS).glob('*.json')),
                    'Acceptance records/pointer exist: in-place hash regeneration refused.')
            structure(root)
            (root / HASHES).write_text(hash_text(identities(root)), encoding='utf-8', newline='\n')
            print('Candidate hashes written. No adoption or source-authenticity claim.')
        else:
            output = args.output.resolve()
            require(output != (root / HASHES).resolve(), 'Candidate output cannot replace the active member list.')
            require(not output.is_relative_to((root / 'project_records/governance').resolve()), 'Candidate output cannot overwrite acceptance storage.')
            structure(root)
            with output.open('x', encoding='utf-8', newline='\n') as f:
                f.write(hash_text(identities(root)))
            print('Separate candidate hashes written; active member list and acceptance unchanged.')
    except (ValueError, OSError, TypeError, KeyError) as exc:
        print('FAIL: ' + str(exc), file=sys.stderr)
        status = 1
    for warning in WARNINGS:
        print('WARN: ' + warning, file=sys.stderr)
    return status


if __name__ == '__main__':
    raise SystemExit(main())
