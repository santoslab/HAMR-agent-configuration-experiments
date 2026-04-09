#![cfg_attr(not(test), no_std)]

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

#![allow(dead_code)]
#![allow(static_mut_refs)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(unused_parens)]
#![allow(unused_unsafe)]
#![allow(unused_variables)]

// This file will not be overwritten if codegen is rerun

use data::*;
use vstd::prelude::*;

macro_rules! implies {
  ($lhs: expr, $rhs: expr) => {
    !$lhs || $rhs
  };
}

macro_rules! impliesL {
  ($lhs: expr, $rhs: expr) => {
    !$lhs | $rhs
  };
}

// BEGIN MARKER GUMBO RUST MARKER
pub fn clampedPayloadLowerBound() -> i32
{
  0i32
}

pub fn clampedPayloadUpperBound() -> i32
{
  100i32
}

pub fn clampedPayload(m: SNG_Data_Model::Message) -> bool
{
  (clampedPayloadLowerBound() <= m.payload) &&
    (m.payload <= clampedPayloadUpperBound())
}

pub fn allowedSecurityLevel(m: SNG_Data_Model::Message) -> bool
{
  (m.security_level == SNG_Data_Model::SecurityLevel::Restricted) ||
    (m.security_level == SNG_Data_Model::SecurityLevel::Public)
}

pub fn equalSecurityLevel(
  m1: SNG_Data_Model::Message,
  m2: SNG_Data_Model::Message) -> bool
{
  m1.security_level == m2.security_level
}

pub fn equalPayload(
  m1: SNG_Data_Model::Message,
  m2: SNG_Data_Model::Message) -> bool
{
  m1.payload == m2.payload
}

pub fn equalMessage(
  m1: SNG_Data_Model::Message,
  m2: SNG_Data_Model::Message) -> bool
{
  equalSecurityLevel(m1, m2) && equalPayload(m1, m2)
}
// END MARKER GUMBO RUST MARKER

verus! {
  // BEGIN MARKER GUMBO VERUS MARKER
  pub open spec fn clampedPayloadLowerBound_spec() -> i32
  {
    0i32
  }

  pub open spec fn clampedPayloadUpperBound_spec() -> i32
  {
    100i32
  }

  pub open spec fn clampedPayload_spec(m: SNG_Data_Model::Message) -> bool
  {
    (clampedPayloadLowerBound_spec() <= m.payload) &&
      (m.payload <= clampedPayloadUpperBound_spec())
  }

  pub open spec fn allowedSecurityLevel_spec(m: SNG_Data_Model::Message) -> bool
  {
    (m.security_level == SNG_Data_Model::SecurityLevel::Restricted) ||
      (m.security_level == SNG_Data_Model::SecurityLevel::Public)
  }

  pub open spec fn equalSecurityLevel_spec(
    m1: SNG_Data_Model::Message,
    m2: SNG_Data_Model::Message) -> bool
  {
    m1.security_level == m2.security_level
  }

  pub open spec fn equalPayload_spec(
    m1: SNG_Data_Model::Message,
    m2: SNG_Data_Model::Message) -> bool
  {
    m1.payload == m2.payload
  }

  pub open spec fn equalMessage_spec(
    m1: SNG_Data_Model::Message,
    m2: SNG_Data_Model::Message) -> bool
  {
    equalSecurityLevel_spec(m1, m2) && equalPayload_spec(m1, m2)
  }
  // END MARKER GUMBO VERUS MARKER
}
