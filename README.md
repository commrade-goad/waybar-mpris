# waybar-mpris-json
a waybar module that will show current active media on json format.

## Build
``` shell
$ cargo build --release
```

## Usage
```
waybar-mpris-json [-i [interval] / -emsg [alternatif message]]
```
in case if this program/script didn't get argument it will fallback to default value of 1000 milisecond / 1 second and '\n' for the alternatif message.

## Example
Here is the example how to implement it on waybar configuration file.

-`~/.config/waybar/config`
``` json
"custom/media": {
    	  "format": " {}",
    	  "escape": true,
          "return-type": "json",
          "max-length": 30,
          "on-click": "playerctl play-pause",
          "on-click-right": "playerctl stop",
          "smooth-scrolling-threshold": 10,
          "on-scroll-up": "playerctl next",
          "on-scroll-down": "playerctl previous",
          "exec": "/path/to/waybar-mpris-json -i 250 -emsg \"Its quiet now...\"",
     },
```
