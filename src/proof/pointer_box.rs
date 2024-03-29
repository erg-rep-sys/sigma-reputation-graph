use std::fmt::{Debug, Formatter};
use crate::proof::reputation_proof::ReputationProof;

#[derive(PartialEq, Debug, Clone)]
pub enum Pointer {
    ReputationProof(ReputationProof),
    String(String)
}

#[derive(Clone)]
pub struct  PointerBox {
    pointer: Pointer,
    box_id: Vec<u8>,
    pub(crate) amount: i64
}

impl<'a> PointerBox {
    pub(crate) fn compute(&self, pointer: Pointer) -> f64 {
        match self.pointer.clone() {
            Pointer::ReputationProof(proof) => if pointer == self.pointer { 1.00 } else { proof.compute(pointer) },
            Pointer::String(..) => if pointer == self.pointer { 1.00 } else { 0.00 }
        }
    }
}

impl<'a> Debug for PointerBox {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "PointerBox box id: {:?}, with amount {}. \n",
               self.box_id, self.amount)
    }
}

impl <'a> PointerBox {
    pub fn new(
        box_id: Vec<u8>,
        amount: i64,
        pointer: Pointer
    ) -> PointerBox {
        PointerBox {
            pointer,
            box_id,
            amount,
        }
    }
}