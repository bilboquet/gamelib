DOTFILES := $(filter-out master.dot, $(wildcard *.dot))
SVGFILES := $(DOTFILES:.dot=.svg)
PNGFILES := $(DOTFILES:.dot=.png)

NODE_SHAPE?=circle

all: $(SVGFILES) $(PNGFILES) master.svg master.png

%.svg: %.dot
	dot -Tsvg -Nshape=$(NODE_SHAPE) -Nwidth=".3" $< -o $@

%.png: %.dot
	dot -Tpng -Nshape=$(NODE_SHAPE) -Nwidth=".3" -Glabel=$(basename $<) -Glabelloc=b $< -o $@

master.svg: master.dot $(SVGFILES)
	dot -Tsvg -Nshape=$(NODE_SHAPE) -Nwidth=".3" -Glabel=$(basename $<) -Glabelloc=b $< -o $@

master.png: master.dot $(PNGFILES)
	dot -Tpng -Nshape=$(NODE_SHAPE) -Nwidth=".3" -Glabel=$(basename $<) -Glabelloc=b $< -o $@

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