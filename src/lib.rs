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

/// Problem
pub trait Problem {
    /// Print the problem statement, including the example.
    fn statement(&self);

    /// Solve the problem with the basic
    fn solve(&self) -> Result<(), String>;
}
