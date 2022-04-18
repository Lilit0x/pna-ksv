use kvs::{cli};
fn main() {
  let matches = cli().get_matches();

  match matches.subcommand() {
    Some(("get", _sub_matches)) => {
      unimplemented!("unimplemented");
    }
    Some(("set", _sub_matches)) => {
      unimplemented!("unimplemented");
    }
    Some(("rm", _sub_matches)) => {
      unimplemented!("unimplemented");
    }
    None => todo!(),
    _ => unimplemented!()
  }

}