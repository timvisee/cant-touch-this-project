---
title: "Can't Touch This - Progress Report Week 3"
subtitle: "A touchless computing interface"
author: Nathan Bakhuijzen & Tim Visée
date: 2018-09-10
categories:
- presentation
- rust
theme: solarized
progress: true
slideNumber: true
history: true
---

# Different approach

## Why?
* Too broad
* Little on gestures
* Project in mind
* Why search for a problem?

> * _More effective:_ Research by trying

## How?
* Start with limited research
* Focus on implementation
* Conduct experiments, produce results

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
* 1 Leap Motion / machine
* Other sensors: difficult to abstract

## SDK & Platforms
* Proprietary SDK
* Proprietary sensor data
* Open-source implementation: limited & broken
* Custom game engine implementation, not usable 

## Gesture libraries
* No libraries for Rust
* Possible usable C implementation

## Gesture recognition
* ML is expensive
* Circular measurements

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
