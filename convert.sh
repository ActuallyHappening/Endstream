for image in *.jpg; do

	convert "$image" "${image%.jpg}.png"

done
