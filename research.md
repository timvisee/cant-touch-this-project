### Preparation research
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
- Is there a usable library available for implementing gesture detection, that
  we can use for our codebase and environment? Is it good enough so we don't
  have to implement a gesture detection system ourselves?
- How to implement gesture based detection based on hand coordinates?

### Build prototype
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
    - Invoke a user defined command
    - ...

### Experiment
- Experiment what gestures can be detected by the Leap Motion?
  How accurate is the detection?
    - Pinching
    - Pointing
    - Pointing with multiple fingers
    - Twisting (opening a bottle, or a rotate motion with two fingers)
    - Tapping your thumb against your index finger
    - Draw circle in various directions
- How effective are hand gestures versus basic hand movements (with thresholds)?
- What is in our opinion the best way to:
    - Move the mouse and left/right click
    - Scroll over a page
- For the above, could it replace a mouse and keyboard or touch in the following
  situation:
  cursor.
    - For common tasks on an office computer
    - In a sterile medical environment for controlling a simple röntgen interface
    - ...
