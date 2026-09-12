// Independently authored vectors by the separated third reviewer. No writer test copy.
#![cfg(test)]
#![allow(unused_imports)]
use spark_core::clock::LogicalTime;
use spark_core::value::{CanonicalValue,FixedPoint,FIXED_SCALE};
use spark_engine::engine::{Engine,EngineGenesis,CompletedHistory};
use spark_engine::request::{CommandRequest,CommandPayload,Request,Outcome};
use spark_engine::report::{CohortOutcome,WaveRejection,CommittedEffect};
use spark_engine::rules::{Update,Param,RuleSpec,ActivatedRuleSet,Trigger,Direction};
use spark_testkit::phase2::*;
fn half()->Update { Update::Scale(FixedPoint::from_raw(FIXED_SCALE/2)) }
fn wide()->Update { Update::Clamp{min:-1000,max:1000} }
fn add(n:i64)->Update { Update::Add(lit(n)) }
fn sub(n:i64)->Update { Update::Subtract(lit(n)) }
fn aggregate()->Update { Update::Aggregate{source:def("state.mood"),weight:FixedPoint::from_raw(FIXED_SCALE)} }
fn dec()->Update { Update::Decay{rate:Param::Config{key:def("tune.rate")},cadence:Param::Config{key:def("tune.cadence")}} }
fn cfg(rate:i64,cadence:i64)->spark_engine::profile::config::ConfigRevision { config(vec![("tune.rate",CanonicalValue::Int(rate)),("tune.cadence",CanonicalValue::Int(cadence))]) }
struct R { e:Engine,g:EngineGenesis,on:ActivatedRuleSet,off:ActivatedRuleSet,n:u64,h:Vec<CommandRequest>,target:String }
impl R {
 fn new(target:&str,stages:Vec<Update>,rate:i64,cadence:i64,pacing:u32,evals:&[u64],watch:Option<(i64,Direction)>)->Self {
 let explicit=stages.iter().any(|s|matches!(s,Update::Decay{..}));
 let body=stages.into_iter().enumerate().map(|(i,u)|emit(&format!("s{i}"),target,u)).collect();
 let mut common=vec![on_command("rule.set","cmd.set",vec![emit("s",target,Update::Assign(param(0)))]),on_command("rule.body","cmd.body",body),on_command("rule.source","cmd.source",vec![emit("s","state.mood",Update::Assign(param(0)))]),on_command("rule.valid","cmd.valid",vec![emit("a",target,Update::Add(param(0))),emit("c",target,wide())])];
 if let Some((threshold,direction))=watch { common.push(rule("rule.watch",Trigger::Crossing{watched:def(target),threshold,direction},vec![emit("a","state.alarm",add(1))])); }
 let mut f=standard();let base=vec![baseline(target,0)];let off=f.rule_set_with(budgets(pacing),common.clone(),base.clone()).unwrap();
 if !explicit { common.push(work_rule("rule.decay","work.decay",vec![emit("d",target,dec())])); }
 let initial=evals.iter().map(|t|initial("rule.decay",&actor("a"),*t,"work.decay")).collect();
 let mut g=f.genesis_with(budgets(pacing),common,base,initial);g.config=cfg(rate,cadence);
 Self{e:Engine::genesis(g.clone()).unwrap(),on:g.rule_set.clone(),off,g,n:0,h:vec![],target:target.into()}
 }
 fn cmd(&mut self,t:u64,k:&str,x:i64)->CommandRequest {self.n+=1;command_request(&format!("cmd.{}",self.n),t,self.n,k,actor("a"),vec![],vec![x])}
 fn execute(&mut self,c:CommandRequest)->(CohortOutcome,Vec<CommittedEffect>) {let rs=drive(&mut self.e,&Request::Command(c.clone()));assert_eq!(rs.last().unwrap().outcome(),&Outcome::Completed);self.h.push(c);let rr=reports(&rs);let effects=rr.iter().flat_map(|r|r.waves.iter().flat_map(|w|w.committed.clone())).collect();(rr.last().unwrap().outcome.clone(),effects)}
 fn send(&mut self,t:u64,k:&str,x:i64)->Vec<CommittedEffect> {let c=self.cmd(t,k,x);let(o,e)=self.execute(c);assert_eq!(o,CohortOutcome::Committed);e}
 fn epoch(&mut self,t:u64,present:bool,rate:i64,cad:i64) {let mut c=self.cmd(t,"spark.epoch.activate",0);c.payload=CommandPayload::ActivateEpoch{rule_set:if present{self.on.clone()}else{self.off.clone()},config:cfg(rate,cad)};assert!(matches!(self.execute(c).0,CohortOutcome::EpochActivated{..}));}
 fn read(&self,d:&str)->Option<(i64,u64)> {self.e.state().get(&profile_id(),&def(d),&actor("a")).map(|c|match c.value{CanonicalValue::Int(x)=>(x,c.updated_at.0),_=>panic!("numeric")})}
 fn want(&self,v:i64,t:u64){assert_eq!(self.read(&self.target),Some((v,t)));}
 fn go(&mut self,t:u64){let r=drive(&mut self.e,&advance(t));assert_eq!(r.last().unwrap().outcome(),&Outcome::Completed);}
 fn restore(&mut self){self.e=Engine::restore(self.e.snapshot().unwrap(),&self.g.profile).unwrap();}
 fn replay(&self){let h=CompletedHistory{commands:self.h.clone(),resets:vec![],frontier:self.e.frontier(),recorded_history_digest:self.e.timeline_history_digest(),recorded_stable_boundary_digest:self.e.stable_boundary_digest()};let other=Engine::reconstruct_completed(self.g.clone(),&h).unwrap();assert_eq!(digest_pair(&self.e),digest_pair(&other));}
}
#[test] fn own_nonidentity_transform_first_subtract_derived(){
 for target in ["state.stress","state.pressure"] {for (initial,delta,want,next) in [(101,3,39,29),(-101,-3,-40,-30)] {
 // floor(settle(101)/2) - 3 - 3 = 39; floor(-91/2)+3+3=-40.
 let mut r=R::new(target,vec![half(),sub(delta),sub(delta),wide()],10,10,1,&[20],None);r.send(0,"cmd.set",initial);r.send(15,"cmd.body",0);r.want(want,15);r.restore();r.go(20);r.want(next,20);r.replay();}}
}
#[test] fn own_multistage_no_double_charge(){for s in [1,-1] {
 // settle, subtract 3*s, double: 100->90->87->174; next writes 342 and 658.
 let mut r=R::new("state.stress",vec![sub(3*s),Update::Scale(FixedPoint::from_raw(FIXED_SCALE*2)),wide()],10,10,1,&[30],None);
 r.send(0,"cmd.set",100*s);r.send(15,"cmd.body",0);r.want(174*s,15);r.send(16,"cmd.body",0);r.want(342*s,16);r.send(25,"cmd.body",0);r.want(658*s,25);r.go(30);r.want(648*s,30);r.replay();}}
#[test] fn own_aggregate_replacement_stage_orders(){for target in ["state.stress","state.pressure"] {for sign in [1,-1] {for (stages,want) in [(vec![aggregate(),sub(3*sign),half()],18*sign-(sign<0) as i64),(vec![half(),sub(3*sign),aggregate()],40*sign)] {
 let mut r=R::new(target,stages,10,10,2,&[20],None);r.send(0,"cmd.set",100*sign);r.send(1,"cmd.source",40*sign);r.send(15,"cmd.body",0);r.want(want,15);r.go(20);r.want(want-10*sign,20);r.replay();}}}}
#[test] fn own_assign_order(){for (stages,want) in [(vec![Update::Assign(lit(-41)),half(),sub(-3)],-18),(vec![sub(3),half(),Update::Assign(lit(41))],41)] {let mut r=R::new("state.pressure",stages,10,10,1,&[],None);r.send(0,"cmd.set",100);r.send(15,"cmd.body",0);r.want(want,15);r.replay();}}
#[test] fn own_declared_decay_order(){for sign in [1,-1] {for (stages,want) in [(vec![sub(-5*sign),dec(),wide()],3*sign),(vec![dec(),wide(),sub(-5*sign)],5*sign)] {let mut r=R::new("state.pressure",stages,3,6,1,&[],None);r.send(5,"cmd.set",4*sign);r.send(12,"cmd.body",0);r.want(want,12);r.replay();}}}
#[test] fn own_transform_boundaries(){for stages in [vec![half()],vec![Update::Clamp{min:-60,max:60}],vec![Update::Clamp{min:-80,max:80},half()]] {for sign in [1,-1] {let expected=if stages.len()>1{40}else if matches!(stages[0],Update::Clamp{..}){60}else{50};let mut r=R::new("state.stress",stages.clone(),10,10,1,&[20],None);r.send(0,"cmd.set",100*sign);r.send(15,"cmd.body",0);r.want(expected*sign,15);r.go(20);r.want((expected-10)*sign,20);r.replay();}}}
#[test] fn own_barrier_and_removed_lineage(){for sign in [1,-1] {let mut r=R::new("state.pressure",vec![wide(),sub(-3*sign)],4,5,1,&[24,27],None);r.send(0,"cmd.set",100*sign);r.epoch(10,true,7,4);r.send(10,"cmd.body",0);r.want(95*sign,10);r.epoch(19,false,7,4);r.send(21,"cmd.body",0);r.want(84*sign,21);r.restore();r.epoch(23,true,7,4);r.go(24);r.want(84*sign,24);r.go(27);r.want(77*sign,27);r.replay();}}
#[test] fn own_refusal_keeps_debt_and_time(){let mut r=R::new("state.bounded",vec![wide(),Update::Subtract(param(0)),half()],1,4,1,&[],None);r.send(0,"cmd.set",9);let c=r.cmd(6,"cmd.body",-20);let(o,e)=r.execute(c);assert!(matches!(o,CohortOutcome::Rejected{rejection:WaveRejection::InvalidEffect{..},..}));assert!(e.is_empty());r.want(9,0);r.restore();r.send(10,"cmd.valid",1);r.want(8,10);r.replay();}
#[test] fn own_watchers_both_directions_single_effect(){for (sign,threshold,direction) in [(1,97,Direction::Falling),(-1,-97,Direction::Rising)] {let mut r=R::new("state.pressure",vec![wide(),sub(-5*sign)],10,10,1,&[],Some((threshold,direction)));r.send(0,"cmd.set",100*sign);let e=r.send(15,"cmd.body",0);r.want(95*sign,15);assert_eq!(r.read("state.alarm"),Some((1,15)));let target:Vec<_>=e.iter().filter(|x|x.definition==def("state.pressure")).collect();assert_eq!(target.len(),1);assert_eq!(target[0].value,CanonicalValue::Int(95*sign));r.replay();}}
#[test] fn own_pacing_restore_replay(){let mut traces=vec![];for pacing in [1,2,3,7,50] {for restore in [false,true] {let mut r=R::new("state.pressure",vec![half(),sub(-3),wide()],3,7,pacing,&[9,31,44],None);let mut t=vec![];r.send(2,"cmd.set",101);r.go(9);for (at,epoch) in [(15,0),(26,1),(38,2)] {if epoch==1{r.epoch(19,true,5,4)}if epoch==2{r.go(31);r.epoch(34,false,5,4)}r.send(at,"cmd.body",0);if restore{r.restore()}t.push(r.read("state.pressure"));}r.epoch(40,true,5,4);r.go(44);t.push(r.read("state.pressure"));r.replay();traces.push(t);}}for t in &traces{assert_eq!(t,&traces[0]);}}

#[test] fn own_same_time_reversal_and_unchanged(){for changed in [false,true] {let mut r=R::new("state.pressure",vec![wide(),sub(-3)],4,5,1,&[14,15],None);r.send(0,"cmd.set",100);if changed{r.epoch(9,true,7,3);}r.epoch(9,true,4,5);r.send(12,"cmd.body",0);r.want(if changed{99}else{95},12);r.go(14);r.want(95,14);r.go(15);r.want(if changed{95}else{91},15);r.replay();}}
#[test] fn own_absence_and_no_move_time(){let mut r=R::new("state.pressure",vec![wide(),sub(-3)],0,5,1,&[5,8],None);r.go(5);assert_eq!(r.read("state.pressure"),None);r.send(6,"cmd.body",0);r.want(3,6);r.go(8);r.want(3,8);r.replay();}
// Added after mutation revealed that a binding Clamp masked over-settlement in v1.
#[test] fn own_nonbinding_transform_only(){for sign in [1,-1]{let mut r=R::new("state.pressure",vec![half(),wide()],10,10,1,&[20],None);r.send(0,"cmd.set",100*sign);r.send(15,"cmd.body",0);r.want(50*sign,15);r.go(20);r.want(40*sign,20);r.replay();}}
