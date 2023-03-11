DOTFILES := $(filter-out master.dot, $(wildcard *.dot))
SVGFILES := $(DOTFILES:.dot=.svg)
PNGFILES := $(DOTFILES:.dot=.png)

NODE_SHAPE?=circle
RESOLUTION?="-Gdpi=150"

all: $(SVGFILES) $(PNGFILES) master.svg master.png master.pdf

%.svg: %.dot
	dot -Tsvg -Nshape=$(NODE_SHAPE) -Nwidth=".3" $< -o $@

%.png: %.dot
	dot -Tpng ${RESOLUTION} -Nshape=$(NODE_SHAPE) -Nwidth=".3" -Glabel=$(basename $<) -Glabelloc=b $< -o $@

%.pdf: %.dot
	dot -Tpdf ${RESOLUTION} -Nshape=$(NODE_SHAPE) -Nwidth=".3" -Glabel=$(basename $<) -Glabelloc=b $< -o $@

master.svg: master.dot $(SVGFILES)
	dot -Tsvg -Nshape=$(NODE_SHAPE) -Nwidth=".3" -Glabel=$(basename $<) -Glabelloc=b $< -o $@

master.png: master.dot $(PNGFILES)
	dot -Tpng ${RESOLUTION} -Nshape=$(NODE_SHAPE) -Nwidth=".3" -Glabel=$(basename $<) -Glabelloc=b $< -o $@

master.pdf: master.dot $(PNGFILES)
	dot -Tpdf ${RESOLUTION} -Nshape=$(NODE_SHAPE) -Nwidth=".3" -Glabel=$(basename $<) -Glabelloc=b $< -o $@

point:
	NODE_SHAPE=point make clean all
	mv master.svg point.svg
	mv master.png point.png

circle:
	NODE_SHAPE=circle make clean all
	mv master.svg circle.svg
	mv master.png circle.png


clean:
	rm -f $(SVGFILES) $(PNGFILES)