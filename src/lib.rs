// coding-problems
// Copyright (C) 2019  Joshua Ellis <josh@jpellis.me>
//
// This program is free software: you can redistribute it and/or modify it under
// the terms of the GNU General Public License as published by the Free Software
// Foundation, either version 3 of the License, or (at your option) any later
// version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE.  See the GNU General Public License for more
// details.
//
// You should have received a copy of the GNU General Public License along with
// this program.  If not, see <http://www.gnu.org/licenses/>.

#![cfg_attr(feature = "nightly", feature(test))]
#[cfg(feature = "nightly")]
extern crate test;

pub mod daily_coding_problem;
pub mod project_euler;

use std::{error, fmt, io, io::prelude::*};

/// Problem
pub trait Problem {
    /// Output the problem's name
    fn name(&self) -> &str;

    /// Output the problem's statement
    fn statement(&self) -> &str;

    /// Solve the problem, writing any information to the `out`.
    fn solve(&self, out: &mut dyn Write) -> Result<(), Error>;
}

////////////////////////////////////////////////////////////////////////////////
// Error
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub enum Error {
    Unimplemented,
    Failed(String),
    Io(io::Error),
}

impl Error {
    /// Return true of the problem failed to produce the expected result.
    ///
    /// This is distinct to other possible errors which meant that the result
    /// could not be computed.
    pub fn is_failed(&self) -> bool {
        match self {
            Self::Failed(..) => true,
            _ => false,
        }
    }

    /// Return true if the problem is not implemented.
    pub fn is_unimplemented(&self) -> bool {
        match self {
            Self::Unimplemented => true,
            _ => false,
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::Unimplemented => write!(f, "not implemented"),
            Self::Failed(s) => write!(f, "{}", s),
            Self::Io(e) => write!(f, "{}", e),
        }
    }
}

impl From<()> for Error {
    fn from(_: ()) -> Self {
        Self::Unimplemented
    }
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Self::Failed(s)
    }
}

impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Self::Failed(s.into())
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}
