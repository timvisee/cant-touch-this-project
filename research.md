Research:
- Is it possible to use multiple Leap Motion sensors, to allow combining sensors
  data?
    - Would better detection accuracy be achieved by combining sensor data?
    - It it possible to use an additional sensor such as the Xbox Kinect without
      too much implementation effort.
- Is using a gesture based interface to control a computer even viable to use
  anyway? Viable defines:
    - Is it more efficient than existing input methods, such as a keyboard and
      mouse. Is interacting faster?
    - Is it usable in the medical world in sterile environments, to replace
      touch based interaction?
  If not, there is no point in implementing such as system.
- Is there a usable library available for implementing gesture detection, so we
  don't have to implement it ourselves?
- How to implement gesture based detection based on hand coordinates?

Build prototype:
- Build a general purpose application, that allows interacting with a computer.
  General purpose in the sense that it is usable on a computer for general
  interaction, such as reading a file or for web browsing.
- Start implementing simple hand movements based on coordinate/position
  thresholds.
- Work in implementing more advanced gesture detection once a first iteration is
  running.
- Configurable actions for hand movements and gestures through a GUI. Think of
  the following actions:
    - Move the mouse and left/right click
    - Scroll
    - Open a file
    - Switch windows
    - Invoke a key press
    - ...

Experiment:
- Experiment what gestures are usable?
- Experiment what gestures can be detected by the Leap Motion (accuracy)
    - Pinching
    - Pointing
    - Twisting
    - Tapping with thumb
    - Draw circle
- Experiment what interactions are replicable with gestures
- What is to our opinion the most effective way of controlling
  an on-screen cursor.
    - Is it working well enough to replace a mouse?
