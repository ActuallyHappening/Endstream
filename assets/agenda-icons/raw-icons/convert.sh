for image in *.svg; do

	rsvg-convert -h 143 "$image" >"${image%.svg}.png"

done
