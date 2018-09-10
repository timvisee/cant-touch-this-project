---
title: "Can't Touch This"
subtitle: "Progress report - Week 3"
author: Nathan Bakhuijzen & Tim Visée
date: 2018-09-10
categories:
- presentation
- rust
theme: solarized
transition: concave
progress: true
slideNumber: true
history: true
---

# Different approach

## How?
* Start with limited research
* Focus on implementation
* Conduct experiments, produce results

## Why?
* Too broad
* Little on gestures
* Project in mind
* Why search for a problem?

> * _More effective:_ Research by trying

# Planning

## Week mapping
* Research almost over
* ~4 week implementation period
* ~3 days for user manual
* ~2 days of experimenting

# Last week

## Research
* Multiple Leap Motion sensors
* ... and other sensors
* SDK possibilities, alternatives
* Gesture recognition libraries
* Gesture recognition methods

# Achievements & Findings

## Sensors
* 1 Leap Motion / machine [<sup>⧉</sup>][1-sensor]
* Other sensors: difficult to abstract

[1-sensor]: https://forums.leapmotion.com/t/multiple-leap-motion-support/770

## SDK & Platforms
* Proprietary SDK 😠 [<sup>⧉</sup>][leap-motion-sdk]
* Proprietary sensor data 😡
* Open-source implementation: limited & broken [<sup>⧉</sup>][open-sdk]
* Custom game engine implementation, not usable 

[leap-motion-sdk]: https://developer.leapmotion.com/sdk/v2
[open-sdk]: https://github.com/openleap

## Gesture libraries
* No libraries for Rust [<sup>⧉</sup>][no-gesture-crate]
* Possible usable C implementation

[no-gesture-crate]: https://crates.io/search?q=gesture

## Gesture recognition
* ML is expensive [<sup>⧉</sup>][ml-is-expensive]
* Circular measurements [<sup>⧉</sup>][ml-is-expensive]

[ml-is-expensive]: https://datascience.stackexchange.com/questions/26209/why-is-training-take-so-long-on-my-gpu
[circular-measurements]: https://www.slideshare.net/RACSOstudentHELP/as-level-circular-measure-geometry-explained

# Next week

## Continue research
* Gesture based interface viable
* Gesture detection methods
* Possible medical applications

## Start implementing
* First abstractions
* Sensor data fetching
* Basic movement detection
* Initial backend for web configuration

# Thanks!

## Any questions?

Slides are available on GitLab at  
[gitlab.com/timvisee/cant-touch-this-project](https://gitlab.com/timvisee/cant-touch-this-project)
