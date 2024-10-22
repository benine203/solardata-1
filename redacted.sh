#!/bin/bash

if test "$1" == ""; then
	echo "Usage: $0 <tucson|seattle> [gnuplot_term] [gnuplot_size]"
	echo "Example: $0 tucson qt 800,600"
	exit 1
fi

GPTERM="qt"
GPTERMSIZE="800,600"

if test -n "$2"; then
	GPTERM="$2"
fi

if test -n "$3"; then
	GPTERMSIZE="$3"
fi

#Fitment Sunrise: SineFitment { period_mul_2pi: 1.0, a: 1.0750000000000002, b: 0.01721420632103996, c: 84.0, d: 6.341666666666667 }
#Fitment Sunset: SineFitment { period_mul_2pi: 1.0, a: 1.1166666666666671, b: 0.01721420632103996, c: -73.0, d: 18.433333333333334 }
#Fitment Solar Noon: SineFitment { period_mul_2pi: 2.0, a: 0.25, b: 0.03442841264207992, c: 155.0, d: 12.366666666666667 }
#Fitment Day Length: SineFitment { period_mul_2pi: 1.0, a: 2.1166666666666663, b: 0.01721420632103996, c: -84.5, d: 12.150000000000002 }
#
#Sunrise f(x) = 1.075 * sin(0.01721420632103996 * x + 84.0 * 1.0 * 2 * pi / 365) + 6.341666666666667
#Latex: $f(x) = 1.075 \sin(0.01721420632103996 x + 84.0 \frac{1 \cdot 2 \pi}{365}) + 6.341666666666667$
#
#Sunset f(x) = 1.116666666666667 * sin(0.01721420632103996 * x - 73.0 * 1.0 * 2 * pi / 365) + 18.433333333333334
#Latex: $f(x) = 1.116666666666667 \sin(0.01721420632103996 x - 73.0 \frac{1 \cdot 2 \pi}{365}) + 18.433333333333334$
#
#Solar Noon f(x) = 0.25 * sin(0.03442841264207992 * x + 155.0 * 2.0 * 2 * pi / 365) + 12.366666666666667
#
#Day Length f(x) = 2.1166666666666663 * sin(0.01721420632103996 * x - 84.5 * 1.0 * 2 * pi / 365) + 12.150000000000002


#with gnuplot:

SETUP="set terminal $GPTERM size $GPTERMSIZE;"
SETUP="$SETUP set samples 500; set grid; set key top left;"
SETUP="$SETUP set xtics 0,30,365; set ytics 0,3,24;"
SETUP="$SETUP set xzeroaxis; set yzeroaxis; set border 0;"
SETUP="$SETUP set xtics nomirror; set ytics nomirror;"
SETUP="$SETUP set xtics out; set ytics out;"
SETUP="$SETUP set xtics rotate by -45;"
SETUP="$SETUP set xtics format '%.0f'; set ytics format '%.0f';"



if test "$1" == "tucson"; then

	PLOTCMD="$SETUP; set title 'Tucson, AZ'; set xlabel 'Day of Year'; set ylabel 'Time of Day'; plot [0:365] [0:24] 1.075 * sin(0.01721420632103996 * x + 84.0 * 1.0 * 2 * pi / 365) + 6.341666666666667 title 'Sunrise', 1.116666666666667 * sin(0.01721420632103996 * x - 73.0 * 1.0 * 2 * pi / 365) + 18.433333333333334 title 'Sunset', 0.25 * sin(0.03442841264207992 * x + 155.0 * 2.0 * 2 * pi / 365) + 12.366666666666667 title 'Solar Noon', 2.1166666666666663 * sin(0.01721420632103996 * x - 84.5 * 1.0 * 2 * pi / 365) + 12.150000000000002 title 'Day Length'"

	echo "$PLOTCMD" | sed 's/;/;\n/g' >&2
	exec gnuplot-qt -e "$PLOTCMD" -p
fi


#Fitment Sunrise: SineFitment { period_mul_2pi: 1.0, a: 1.383333333333335, b: 0.01721420632103996, c: 102.0, d: 6.583333333333336 }
#Fitment Sunset: SineFitment { period_mul_2pi: 1.0, a: 2.4083333333333012, b: 0.01721420632103996, c: -95.5, d: 18.775 }
#Fitment Solar Noon: SineFitment { period_mul_2pi: 2.0, a: 1.0499999999999998, b: 0.03442841264207992, c: -186.75, d: 12.933333333333302 }
#Fitment Day Length: SineFitment { period_mul_2pi: 1.0, a: 3.7749999999999666, b: 0.01721420632103996, c: -77.5, d: 12.208333333333336 }
#
# Sunrise f(x) = 1.383 * sin(0.0172 * x + 102.0 * 1.0 * 2 * pi / 365) + 6.583
# Sunrise A=1.383, B=0.0172, C=102.0, D=6.583
#
# Sunset f(x) = 2.408 * sin(0.0172 * x - 95.5 * 1.0 * 2 * pi / 365) + 18.775
# Sunset A=2.408, B=0.0172, C=-95.5, D=18.775
#
# Solar Noon f(x) = 1.05 * sin(0.0344 * x - 186.75 * 2.0 * 2 * pi / 365) + 12.933
# Solar Noon A=1.05, B=0.0344, C=-186.75, D=12.933
#
# Day Length f(x) = 3.775 * sin(0.0172 * x - 77.5 * 1.0 * 2 * pi / 365) + 12.208
# Day Length A=3.775, B=0.0172, C=-77.5, D=12.208
#
#
if test "$1" == "seattle"; then
	PLTCMD="$SETUP; set title 'Seattle, WA'; set xlabel 'Day of Year'; set ylabel 'Time of Day'; plot [0:365] [0:24] 1.383333333333335 * sin(0.01721420632103996 * x + 102.0 * 1.0 * 2 * pi / 365) + 6.583333333333336 title 'Sunrise', 2.4083333333333012 * sin(0.01721420632103996 * x - 95.5 * 1.0 * 2 * pi / 365) + 18.775 title 'Sunset', 1.0499999999999998 * sin(0.03442841264207992 * x - 186.75 * 2.0 * 2 * pi / 365) + 12.933333333333302 title 'Solar Noon', 3.7749999999999666 * sin(0.01721420632103996 * x - 77.5 * 1.0 * 2 * pi / 365) + 12.208333333333336 title 'Day Length'"

	echo "$PLTCMD" | sed 's/;/;\n/g' >&2
	exec gnuplot-qt -e "$PLTCMD" -p
fi

