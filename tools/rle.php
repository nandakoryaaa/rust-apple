<?php
/**
 * Generate RLE compressed sprite data from png image
 * Generated data is output as RUST static structure to include in the project
 * Redirect stdout to file to save the output
 */
const ALPHA_OPAQUE = 0;

if ($argc < 2) {
	die("usage: rle.php filename.png\n");
}

$filename = $argv[1];
$img = imagecreatefrompng($filename);

if (!$img) {
	die("not a png image: $filename\n");
}

$w = imagesx($img);
$h = imagesy($img);

$color_lut = [];
$linear = [];
$index = 0;

for ($y = 0; $y < $h; $y++) {
	for ($x = 0; $x < $w; $x++) {
		$color = imagecolorat($img, $x, $y);
		$alpha = ($color >> 24) & 255;

		if ($alpha == ALPHA_OPAQUE) {
			if (!isset($color_lut[$color])) {
				$color_lut[$color] = ++$index;
			}
 
			$linear[] = $color_lut[$color];
		} else {
			$linear[] = 0;
		}
	}
}

$linear_len = count($linear);
if ($linear_len != $w * $h) {
	echo "*** Warning: linear len $linear_len is not equal to actual len " . ($w * $h) . ", RLE data may be corrupt\n";
}
$len = 1;
$index = $linear[0];
$rle = [];

for ($i = 1; $i < $linear_len; $i++) {
	if ($index != $linear[$i]) {
		encode($rle, $index, $len);
		$index = $linear[$i];
		$len = 1;
	} else {
		$len++;
	}
}

encode($rle, $index, $len);

$imgout = imagecreatetruecolor($w, $h);
imagefilledrectangle($imgout, 0, 0, $w - 1, $h - 1, 0xffffff);

$palette = array_flip($color_lut);
$x = 0;
$y = 0;
$pos = 0;
$rle_len = count($rle);

echo 'pub static PALETTE:[Color;' . count($palette) . "] = [\n";
foreach ($palette as $rgb) {
	echo "\tColor::RGB(" . ($rgb & 255) . ',' . (($rgb >> 8) & 255) . ',' . (($rgb >> 16) & 255) . "),\n";
}
echo "];\n";
echo "// w: $w, h:$h\n";

list($name, $ext) = explode('.', $filename);
echo 'static ' . strtoupper($name) . ':[u8;' . count($rle) . '] = [' . implode(',', $rle) . "];\n";

/* Uncomment to generate test image from RLE data and save as out.png

while ($pos < $rle_len) {
	$index = $rle[$pos++];
	if ($index & 128) {
		$index &= ~128;
		$len = $rle[$pos++];
	} else {
		$len = 1;
	}
	if (!$index) {
		$x += $len;
		$y += floor($x / $w);
		$x %= $w;
	} else {
		$color = $palette[$index];
		while($len) {
			$limit = $w - $x;
			$chunk = $limit < $len ? $limit : $len;
			imagefilledrectangle(
				$imgout, $x, $y, $x + $chunk - 1, $y, $color
			);
			$len -= $chunk;
			$x += $chunk;
			if ($x >= $w) {
				$y++;
				$x = 0;
			}
		}
	}
}

imagepng($imgout, 'out.png');

*/

function encode(& $rle, $index, $len)
{
	if ($len == 1) {
		$rle[] = $index;
		return;
	}

	$index |= 128;
	while ($len) {
		$chunk = $len >= 255 ? 255 : $len;
		$rle[] = $index;
		$rle[] = $chunk;
		$len -= $chunk;
	}
}
