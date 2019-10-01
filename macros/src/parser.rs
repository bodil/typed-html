// We REALLY don't want to lint the generated parser code.
#![allow(clippy::all)]

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub grammar);
