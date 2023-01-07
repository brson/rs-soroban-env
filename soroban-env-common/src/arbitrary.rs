// TODO:
//
// Think about weird but constructable types:
//
// - symbols containing interior nuls
// - statics with unassigned values
// - statuses with unassigned types
//   - these can be constructed in rust with arbitrary codes that aren't valid per the XDR spec
//
// These probably can't be constructed safely,
// and an attacker would need to go through the XDR type conversions to create them,
// so they probably aren't constructable in practice.
//
// Do cross-contract calls go through XDR serialization?
//
// Should these all be constructed from XDR types for max correctness?

#![cfg(feature = "arbitrary")]

use crate::symbol::Symbol;
use crate::{RawVal, BitSet, Static, Status};
use arbitrary::{Arbitrary, Unstructured};
use crate::xdr::{ScStatic, ScStatus};

impl<'a> Arbitrary<'a> for Symbol {
    fn arbitrary(u: &mut Unstructured<'a>) -> arbitrary::Result<Self> {
        let choices = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_";
        let choices: Vec<char> = choices.chars().collect();

        let mut buf = String::new();

        for _ in 0..10 {
            let choice = u.choose(&choices);
            match choice {
                Ok(ch) => {
                    buf.push(*ch);
                }
                Err(_) => {
                    break;
                }
            }
        }

        let symbol = Symbol::try_from_str(&buf).expect("Symbol");

        Ok(symbol)
    }
}

impl<'a> Arbitrary<'a> for BitSet {
    fn arbitrary(u: &mut Unstructured<'a>) -> arbitrary::Result<Self> {
        let bits = u64::arbitrary(u)?;
        let bits = bits & 0x0fff_ffff_ffff_ffff;

        let bitset = BitSet::try_from_u64(bits).expect("BitSet");

        Ok(bitset)
    }
}

impl<'a> Arbitrary<'a> for Static {
    fn arbitrary(u: &mut Unstructured<'a>) -> arbitrary::Result<Self> {
        let scval = ScStatic::arbitrary(u)?;
        let rawval = RawVal::from_other_static(scval);
        let staticval = Static::try_from(rawval).expect("Static");
        Ok(staticval)
    }
}

impl<'a> Arbitrary<'a> for Status {
    fn arbitrary(u: &mut Unstructured<'a>) -> arbitrary::Result<Self> {
        // todo: should we create status's with invalid type/code combos?
        let scstatus = ScStatus::arbitrary(u)?;
        let status = Status::from(scstatus);

        Ok(status)
    }
}
