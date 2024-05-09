# hdnimg - hidden in plain sight
this is a small tool for adding bytes in images by changing the least significant bit, which will not change the appearance of the image   
* the bit count can be manually changed, but be careful setting the bit_count to 8 (the max) cause this will effectively make the tool replace the image pixels.

## Example
<table>
  <tr>
    <td>source image</td>
    <td>altered image</td>
  </tr>
  <tr>
    <td><img src="https://github.com/reezey/hdnimg/blob/master/input.png?raw=true" alt="source image" width=128px height=128px></td>
    <td><img src="https://github.com/reezey/hdnimg/blob/master/output.png?raw=true" alt="altered image" width=128px height=128px></td>
  </tr>
</table>

* Notice how these images look identical to the naked eye, yet they are different on a bit basis.
