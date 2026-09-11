from pathlib import Path
import tempfile,subprocess,json
root=Path(__file__).resolve().parents[3]; ev=root/'engineering/phase2/v3_f01_rev2_bounded_correction_evidence_2026-09-10'
extra=r'''
fn independent_checks() {
    let mut checked = 0;
    for prior in 1..=8 {
        let g = SourceId::new("sequencer").unwrap();
        let mut base = fresh(2);
        let mut a = req("prior.s", prior); a.source = "s";
        rev2(&mut base, &g, &a).unwrap();
        let mut b = req("prior.t", prior + 2); b.source = "t";
        rev2(&mut base, &g, &b).unwrap();
        base.reset_epoch(&g, TimelineEpoch(1), SourceId::new("seq2").unwrap()).unwrap();
        let g2 = SourceId::new("seq2").unwrap();
        base.reset_epoch(&g2, TimelineEpoch(2), SourceId::new("seq3").unwrap()).unwrap();
        let g3 = SourceId::new("seq3").unwrap();
        for profile in ["probe", "wrong"] { for epoch in [0, 2] {
          for source in ["s", "t"] { for id in ["prior.s", "prior.t", "new"] {
            for seq in 0..=12 {
                let r = Req { command_id: id, profile, epoch, source, seq };
                let mut actual = base.clone(); let before = snap(&actual);
                let mut reference = base.clone();
                let got = rev2(&mut actual, &g3, &r);
                let want = old(&mut reference, &g3, &r);
                assert_eq!(got.is_ok(), want.is_ok());
                if got.is_err() { assert_eq!(before, snap(&actual)); }
                else { assert_eq!(snap(&actual), snap(&reference)); }
                checked += 1;
            }
          }}
        }}
        // Same frozen frontier: correct reset order reproduces full state;
        // reverse order fails authority/epoch validation and cannot reproduce it.
        let mut replay = fresh(2);
        rev2(&mut replay, &g, &a).unwrap(); rev2(&mut replay, &g, &b).unwrap();
        replay.reset_epoch(&g, TimelineEpoch(1), g2.clone()).unwrap();
        replay.reset_epoch(&g2, TimelineEpoch(2), g3.clone()).unwrap();
        assert_eq!(snap(&base), snap(&replay));
        let mut reversed = fresh(2);
        rev2(&mut reversed, &g, &a).unwrap(); rev2(&mut reversed, &g, &b).unwrap();
        reversed.reset_epoch(&g, TimelineEpoch(2), g3.clone()).unwrap();
        assert!(reversed.reset_epoch(&g3, TimelineEpoch(1), g2.clone()).is_err());
        assert_ne!(snap(&base), snap(&reversed));
    }
    println!("INDEPENDENT: {checked} clean-state differential cases pass across two sources, two resets at one frontier, identity/profile/epoch/sequence variations; full-state refusal and success equality");
    println!("INDEPENDENT: ascending same-frontier reset order passes; reverse-order negative control fails (8 fixtures)");
}
'''
src=(ev/'rev2_finalization_probe.rs').read_text()
# The probe's genesis helper is inspected before running this augmentation.
src=src.replace('fn main() {','fn main() {\n    independent_checks();')+extra
with tempfile.TemporaryDirectory(prefix='spark-independent-probe-') as d:
 p=Path(d);(p/'src').mkdir();(p/'src/main.rs').write_text(src)
 (p/'Cargo.toml').write_text('[package]\nname="spark-independent-probe"\nversion="0.0.0"\nedition="2021"\n[dependencies]\nspark-core={path='+json.dumps(str(root/'crates/spark-core'))+',features=["test-support"]}\n')
 (p/'Cargo.lock').write_bytes((root/'Cargo.lock').read_bytes())
 subprocess.run(['cargo','run','--offline','--quiet','--manifest-path',str(p/'Cargo.toml')],check=True)
