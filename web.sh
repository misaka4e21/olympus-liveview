#!/bin/bash
curl 'http://192.168.0.10/get_caminfo.cgi' 1>&2
curl 'http://192.168.0.10/switch_cammode.cgi?mode=play' 1>&2
curl 'http://192.168.0.10/switch_cammode.cgi?mode=rec&lvqty=0640x0480' 1>&2
curl 'http://192.168.0.10/exec_takemisc.cgi?com=startliveview&port=23333' 1>&2
echo 'test' 1>&2
./main | ffmpeg -re -f image2pipe -c:v mjpeg -i - -f v4l2 -pix_fmt yuv420p /dev/video0
curl 'http://192.168.0.10/exec_takemisc.cgi?com=stopliveview&port=23333' 1>&2
# rm test.mjpeg
