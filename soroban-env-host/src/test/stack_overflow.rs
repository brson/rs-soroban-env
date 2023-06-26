use crate::{
    Host, HostError,
};
use crate::xdr::{ScVec, ScVal};

#[test]
fn deep_vec() -> Result<(), HostError> {
    let host = Host::default();

    let mut v = ScVec::default();
    for _ in 0..1000 {
        let vv = ScVec::try_from(vec![ScVal::from(v)]).unwrap();
        v = vv;
    }

    // stack overflow here
    let _ = host.to_host_val(&ScVal::from(v));

    Ok(())
}
