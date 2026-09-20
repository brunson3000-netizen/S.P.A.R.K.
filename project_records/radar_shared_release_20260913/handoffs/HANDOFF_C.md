# Agent C handoff — RADAR shared package

Use release manifest:
`/home/chromikey/Projects/SPARK/project_records/radar_shared_release_20260913/RADAR_SHARED_RELEASE_MANIFEST.json`

Shared frozen inputs:

- Subject pack: `/home/chromikey/Projects/SPARK/project_records/radar_shared_release_20260913/v1_1_package/RADAR_V1_1_SUBJECT_FIXTURES.json`
- Scorer contract: `/home/chromikey/Projects/SPARK/project_records/radar_shared_release_20260913/v1_1_package/RADAR_V1_1_SCORER_CONTRACT.json`
- Private fixture pack: scorer custody only, same directory.

Assignment is slots 11–15 exactly as recorded in the manifest. Preserve these
assignments. Slot 14 records STRUCTURED_OUTPUT as UNSUPPORTED; slot 15 records
TOOL_USE as UNSUPPORTED. Their other supported dimensions still run.

Offline verification command:

```bash
cd /home/chromikey/Projects/SPARK
/home/chromikey/AI/.venv/bin/python project_records/radar_shared_release_20260913/radar_mission.py validate --out project_records/radar_shared_release_20260913/v1_1_package
cd /home/chromikey/SWARM/worktrees/compute-qualification-toolkit-v1 && PYTHONPATH=. /home/chromikey/AI/.venv/bin/python -m unittest tests.tools.test_radar_battery_v1 tests.tools.test_compute_qualification
cd /home/chromikey/Projects/SPARK
/home/chromikey/AI/.venv/bin/python project_records/radar_shared_release_20260913/radar_mission.py run-lane --manifest project_records/radar_shared_release_20260913/RADAR_SHARED_RELEASE_MANIFEST.json --lane C --out /home/chromikey/SWARM/research/radar_lane_c_20260913
```

Your prior 0/600, `$0`, and no-reservation state is preserved. No budget
re-request is needed; C's `$4.00` allocation remains reserved by the
Operator's authorization.
