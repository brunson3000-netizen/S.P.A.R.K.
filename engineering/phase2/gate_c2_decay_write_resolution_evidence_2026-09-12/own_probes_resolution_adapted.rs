//! Independent hand-valued discriminators; no engine arithmetic or writer model reused.
#![cfg(test)]
use spark_core::{value::CanonicalValue};
use spark_engine::{engine::{Engine,EngineGenesis,CompletedHistory}, request::{Request,CommandPayload,CommandRequest,Outcome}, report::CohortOutcome, rules::*};
use spark_testkit::phase2::*;

struct Rig { e: Engine, g: EngineGenesis, present: ActivatedRuleSet, absent: ActivatedRuleSet, seq:u64, history:Vec<CommandRequest> }
fn cfg(r:i64,c:i64)->spark_engine::profile::config::ConfigRevision {
 config(vec![("tune.rate",CanonicalValue::Int(r)),("tune.cadence",CanonicalValue::Int(c))])
}
impl Rig {
 fn new(rate:i64,cadence:i64,times:&[u64],body:bool,base:i64)->Self {
  let mut f=standard();
  let set=on_command("rule.set","cmd.set",vec![emit("s","state.stress",Update::Assign(param(0)))]);
  let shock=on_command("rule.shock","cmd.shock",vec![emit("s","state.stress",Update::Add(param(0)))]);
  let mut ops=vec![emit("d","state.stress",Update::Decay{rate:Param::Config{key:def("tune.rate")},cadence:Param::Config{key:def("tune.cadence")}})];
  if body { ops.insert(0,emit("a","state.stress",Update::Add(lit(5)))); }
  let d=work_rule("rule.d","work.d",ops);
  let absent=f.rule_set_with(budgets(50),vec![set.clone(),shock.clone()],vec![baseline("state.stress",base)]).unwrap();
  let mut g=f.genesis_with(budgets(50),vec![set,shock,d],vec![baseline("state.stress",base)],times.iter().map(|t|initial("rule.d",&actor("a"),*t,"work.d")).collect());
  g.config=cfg(rate,cadence);
  Self{e:Engine::genesis(g.clone()).unwrap(),present:g.rule_set.clone(),absent,g,seq:0,history:vec![]}
 }
 fn send(&mut self,t:u64,kind:&str,arg:i64,activation:Option<(bool,i64,i64)>) {
  self.seq+=1;
  let mut c=command_request(&format!("cmd.{}",self.seq),t,self.seq,kind,actor("a"),vec![],vec![arg]);
  if let Some((present,r,cad))=activation {c.payload=CommandPayload::ActivateEpoch{rule_set:if present {self.present.clone()}else{self.absent.clone()},config:cfg(r,cad)};}
  let out=drive(&mut self.e,&Request::Command(c.clone()));
  assert_eq!(out.last().unwrap().outcome(),&Outcome::Completed);
  let last=reports(&out).last().unwrap().outcome.clone();
  if activation.is_some(){assert!(matches!(last,CohortOutcome::EpochActivated{..}));}else{assert_eq!(last,CohortOutcome::Committed);}
  self.history.push(c);
 }
 fn set(&mut self,t:u64,v:i64){self.send(t,"cmd.set",v,None)}
 fn shock(&mut self,t:u64,v:i64){self.send(t,"cmd.shock",v,None)}
 fn activate(&mut self,t:u64,r:i64,c:i64){self.send(t,"spark.epoch.activate",0,Some((true,r,c)))}
 fn go(&mut self,t:u64){let out=drive(&mut self.e,&advance(t));assert_eq!(out.last().unwrap().outcome(),&Outcome::Completed);}
 fn cell(&self)->Option<(i64,u64)>{self.e.state().get(&profile_id(),&def("state.stress"),&actor("a")).map(|c|match c.value {CanonicalValue::Int(v)=>(v,c.updated_at.0),_=>panic!("numeric")})}
 fn want(&self,v:i64,t:u64){assert_eq!(self.cell(),Some((v,t)));}
 fn restore(&mut self){self.e=Engine::restore(self.e.snapshot().unwrap(),&self.g.profile).unwrap();}
 fn replay(&self){let h=CompletedHistory{commands:self.history.clone(),resets:vec![],frontier:self.e.frontier(),recorded_history_digest:self.e.timeline_history_digest(),recorded_stable_boundary_digest:self.e.stable_boundary_digest()};let e=Engine::reconstruct_completed(self.g.clone(),&h).unwrap();assert_eq!(digest_pair(&e),digest_pair(&self.e));}
}

#[test] fn own_d1_d2_unaligned_write_and_partition(){
 for sign in [-1,1]{for times in [vec![12],vec![6,7,11,12]]{
  let mut r=Rig::new(3,6,&times,false,0);r.set(5,20*sign);r.go(12);r.want(14*sign,12);r.replay();
 }}
}
#[test] fn own_d2_body_and_command_shock(){
 let mut b=Rig::new(3,6,&[6,12],true,0);b.set(5,20);b.go(6);b.want(22,6);b.go(12);b.want(24,12);
 for sign in [-1,1]{let mut r=Rig::new(3,6,&[6,12],false,0);r.set(0,20*sign);r.go(6);r.shock(11,5*sign);r.go(12);r.want(19*sign,12);}
}
#[test] fn own_d3_d4_d5_barrier_endpoint_residual_and_unchanged(){
 for sign in [-1,1]{
  for (barrier,first,expected) in [(12,17,87),(13,18,87)]{
   let mut r=Rig::new(2,4,&[first-1,first],false,0);r.set(0,100*sign);r.activate(barrier,7,5);r.go(first-1);r.want(94*sign,first-1);r.restore();r.go(first);r.want(expected*sign,first);r.replay();
  }
  let mut cadence=Rig::new(2,4,&[16,18],false,0);cadence.set(0,100*sign);cadence.activate(13,2,5);cadence.go(16);cadence.want(94*sign,16);cadence.go(18);cadence.want(92*sign,18);
  let mut same=Rig::new(2,4,&[16],false,0);same.set(0,100*sign);same.activate(13,2,4);same.go(16);same.want(92*sign,16);
  let mut short=Rig::new(1,20,&[11,12],false,0);short.set(0,100*sign);short.activate(7,9,5);short.go(11);short.want(100*sign,11);short.go(12);short.want(91*sign,12);
  let mut long=Rig::new(2,4,&[32,33],false,0);long.set(0,100*sign);long.activate(13,9,20);long.go(32);long.want(94*sign,32);long.go(33);long.want(85*sign,33);
 }
}
#[test] fn own_d4_scheduled_at_barrier_and_later_command(){
 let mut r=Rig::new(2,4,&[12,17],false,0);r.set(0,100);r.activate(12,7,5);r.want(94,12);r.shock(12,5);r.want(99,12);r.go(17);r.want(92,17);
}
#[test] fn own_d6_d7_nonzero_baseline_saturation_and_zero(){
 for sign in [-1,1]{let base=11;let mut r=Rig::new(4,6,&[5,6,12,19],false,base);r.set(1,base+sign*7);r.go(5);r.want(base+sign*7,5);r.go(6);r.want(base+sign*3,6);r.go(12);r.want(base,12);r.go(19);r.want(base,19);r.replay();}
 let mut z=Rig::new(0,6,&[5,12],false,0);z.set(1,70);z.go(5);z.want(70,5);z.go(12);z.want(70,12);
}
#[test] fn own_open_lost_prewrite_step(){
 for sign in [-1,1]{for (evals,want) in [(vec![20],85),(vec![10,20],85)]{
  let mut r=Rig::new(10,10,&evals,false,0);r.set(0,100*sign);r.shock(15,5*sign);r.want((want+10)*sign,15);r.go(20);r.want(want*sign,20);r.replay();
 }}
}
#[test] fn own_open_remove_restore(){
 for sign in [-1,1]{let mut r=Rig::new(2,4,&[25,26],false,0);r.set(0,100*sign);
  r.send(13,"spark.epoch.activate",0,Some((false,2,4)));r.restore();r.activate(22,2,4);r.go(25);r.want(94*sign,25);r.go(26);r.want(92*sign,26);r.replay();}
}
#[test] fn own_open_same_time_activations(){
 for sign in [-1,1]{for changed in [false,true]{
  let mut r=Rig::new(2,4,&[16,17],false,0);r.set(0,100*sign);r.activate(13,if changed{7}else{2},if changed{5}else{4});r.restore();r.activate(13,2,4);r.restore();r.go(16);r.want(if changed{94*sign}else{92*sign},16);r.go(17);r.want(92*sign,17);r.replay();
 }}
}
#[test] fn own_open_absent_cell_no_write(){
 let mut r=Rig::new(2,4,&[4,8],false,0);r.go(4);assert_eq!(r.cell(),None);r.restore();r.set(7,100);r.go(8);r.want(98,8);r.replay();
}
