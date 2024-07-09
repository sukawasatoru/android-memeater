/*
 * Copyright 2024 sukawasatoru
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::prelude::Fallible;
use anyhow::{bail, Context};
use std::time::Duration;

pub mod prelude;

pub fn convert_interval(value: &str) -> Fallible<Duration> {
    let value = value.trim();
    if value.ends_with("ms") {
        let mut segments = value.split("ms");
        let secs = segments
            .next()
            .expect("u64 for seconds")
            .parse::<u64>()
            .context("unexpected format")?;
        Ok(Duration::from_millis(secs))
    } else if value.ends_with('s') {
        let mut seguments = value.split('s');
        let millis = seguments
            .next()
            .expect("u64 for millis")
            .parse::<u64>()
            .context("unexpected format")?;
        Ok(Duration::from_secs(millis))
    } else {
        bail!("e.g. `1s` / `1000ms`")
    }
}

pub fn convert_si_size(value: &str) -> Fallible<usize> {
    let value = value.trim().to_lowercase();
    if value.ends_with("gib") {
        let mut segments = value.split("gib");
        Ok(segments.next().expect("usize for bytes").parse::<usize>()? << 30)
    } else if value.ends_with("gb") {
        let mut segments = value.split("gb");
        Ok(segments.next().expect("usize for bytes").parse::<usize>()? * 1000 * 1000 * 1000)
    } else if value.ends_with('g') {
        let mut segments = value.split('g');
        Ok(segments.next().expect("usize for bytes").parse::<usize>()? * 1000 * 1000 * 1000)
    } else if value.ends_with("mib") {
        let mut segments = value.split("mib");
        Ok(segments.next().expect("usize for bytes").parse::<usize>()? << 20)
    } else if value.ends_with("mb") {
        let mut segments = value.split("mb");
        Ok(segments.next().expect("usize for bytes").parse::<usize>()? * 1000 * 1000)
    } else if value.ends_with('m') {
        let mut segments = value.split('m');
        Ok(segments.next().expect("usize for bytes").parse::<usize>()? * 1000 * 1000)
    } else if value.ends_with("kib") {
        let mut segments = value.split("kib");
        Ok(segments.next().expect("usize for bytes").parse::<usize>()? << 10)
    } else if value.ends_with("kb") {
        let mut segments = value.split("kb");
        Ok(segments.next().expect("usize for bytes").parse::<usize>()? * 1000)
    } else if value.ends_with('k') {
        let mut segments = value.split('k');
        Ok(segments.next().expect("usize for bytes").parse::<usize>()? * 1000)
    } else if value.ends_with('b') {
        let mut segments = value.split('b');
        Ok(segments.next().expect("usize for bytes").parse::<usize>()?)
    } else {
        Ok(value.parse::<usize>()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_interval_sec() {
        let actual = convert_interval("10s").unwrap();

        assert_eq!(actual, Duration::from_secs(10));
    }

    #[test]
    fn convert_interval_millis() {
        let actual = convert_interval("500ms").unwrap();

        assert_eq!(actual, Duration::from_millis(500));
    }

    #[test]
    fn convert_si_size_gb() {
        let actual = convert_si_size("10GB").unwrap();
        assert_eq!(actual, 10 * 1000 * 1000 * 1000);

        let actual = convert_si_size("10g").unwrap();
        assert_eq!(actual, 10 * 1000 * 1000 * 1000);

        let actual = convert_si_size("10gb").unwrap();
        assert_eq!(actual, 10 * 1000 * 1000 * 1000);
    }

    #[test]
    fn convert_si_size_gib() {
        let actual = convert_si_size("10GiB").unwrap();
        assert_eq!(actual, 10 * 1024 * 1024 * 1024);

        let actual = convert_si_size("10gib").unwrap();
        assert_eq!(actual, 10 * 1024 * 1024 * 1024);
    }

    #[test]
    fn convert_si_size_mb() {
        let actual = convert_si_size("10MB").unwrap();
        assert_eq!(actual, 10 * 1000 * 1000);

        let actual = convert_si_size("10m").unwrap();
        assert_eq!(actual, 10 * 1000 * 1000);

        let actual = convert_si_size("10mb").unwrap();
        assert_eq!(actual, 10 * 1000 * 1000);
    }

    #[test]
    fn convert_si_size_mib() {
        let actual = convert_si_size("10MiB").unwrap();
        assert_eq!(actual, 10 * 1024 * 1024);

        let actual = convert_si_size("10mib").unwrap();
        assert_eq!(actual, 10 * 1024 * 1024);
    }

    #[test]
    fn convert_si_size_kib() {
        let actual = convert_si_size("10KB").unwrap();
        assert_eq!(actual, 10 * 1000);

        let actual = convert_si_size("10k").unwrap();
        assert_eq!(actual, 10 * 1000);

        let actual = convert_si_size("10kb").unwrap();
        assert_eq!(actual, 10 * 1000);
    }

    #[test]
    fn convert_si_size_kb() {
        let actual = convert_si_size("10KiB").unwrap();
        assert_eq!(actual, 10 * 1024);

        let actual = convert_si_size("10kib").unwrap();
        assert_eq!(actual, 10 * 1024);
    }

    #[test]
    fn convert_size_b() {
        let actual = convert_si_size("1B").unwrap();
        assert_eq!(actual, 1);

        let actual = convert_si_size("1b").unwrap();
        assert_eq!(actual, 1);

        let actual = convert_si_size("1").unwrap();
        assert_eq!(actual, 1);
    }

    #[test]
    fn convert_size_wo_unit() {
        let actual = convert_si_size("123").unwrap();
        assert_eq!(actual, 123);
    }
}
