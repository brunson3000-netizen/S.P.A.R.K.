# Agent B handoff — RADAR shared package

Use release manifest:
`/home/chromikey/Projects/SPARK/project_records/radar_shared_release_20260913/RADAR_SHARED_RELEASE_MANIFEST.json`

Shared frozen inputs:

- Subject pack: `/home/chromikey/Projects/SPARK/project_records/radar_shared_release_20260913/v1_1_package/RADAR_V1_1_SUBJECT_FIXTURES.json`
- Scorer contract: `/home/chromikey/Projects/SPARK/project_records/radar_shared_release_20260913/v1_1_package/RADAR_V1_1_SCORER_CONTRACT.json`
- Private fixture pack: scorer custody only, same directory.

Assignment is slots 06–10 exactly as recorded in the manifest. Run the
mission-local runner after the independent-review record is present. It
reserves each worst-case attempt in the shared ledger, pins the route, and
records unsupported capability cases without charging them.

Offline verification command:

```bash
cd /home/chromikey/Projects/SPARK
/home/chromikey/AI/.venv/bin/python project_records/radar_shared_release_20260913/radar_mission.py validate --out project_records/radar_shared_release_20260913/v1_1_package
cd /home/chromikey/SWARM/worktrees/compute-qualification-toolkit-v1 && PYTHONPATH=. /home/chromikey/AI/.venv/bin/python -m unittest tests.tools.test_radar_battery_v1 tests.tools.test_compute_qualification
cd /home/chromikey/Projects/SPARK
/home/chromikey/AI/.venv/bin/python project_records/radar_shared_release_20260913/radar_mission.py run-lane --manifest project_records/radar_shared_release_20260913/RADAR_SHARED_RELEASE_MANIFEST.json --lane B --out /home/chromikey/SWARM/research/radar_lane_b_20260913
```

No budget re-request is needed; B's `$4.00` allocation remains reserved by
the Operator's authorization.
