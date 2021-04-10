# Battery Pal

Battery Pal is a windows program to show your battery status with colors and a face. Inspired by [Battery Buddy](https://batterybuddy.app/) which does this much better for macOS.

This is primarily a learning excercise for me with using [Native Windows GUI](https://github.com/gabdube/native-windows-gui) and [Windows-RS](https://github.com/microsoft/windows-rs).


## Usage
Run the program
On a machine with a battery it will select the icon based on battery left over.
| State | Percent Battery|
|:--- | :---: |
| Green | 60 - 100
| Yellow | 20 - 59 |
| Red | 0 - 20 |

On a machine without a battery it just cycles through the icons every 2 seconds.

You can exit by clicking (right or left) and selecting exit.

### The icons are rough, can we use a different set?
Yes, create a new icon set and replace HighBatt.ico,LowBatt.ico and MidBatt.ico in the icons folder with the icons you would like to use and relaunch the program.
32x32 should be all you need currently, but I'm hoping to add a dialog which will use a larger 256x256 icon.

### I'm not showing the icon, just a red X icon
The icons folder needs to be in the same directory as battery pal.
