
// cmdline opts
import { program } from 'commander';

program.option('-l, --location <location>', 'Location to predict sunrise, sunset, solar noon, and day length for');

program.parse();
const options = program.opts();

// TUCSON, AZ
// Fitment Sunrise: SineFitment { period_mul_2pi: 1.0, a: 1.0750000000000002, b: 0.01721420632103996, c: 84.0, d: 6.341666666666667 }
// Fitment Sunset: SineFitment { period_mul_2pi: 1.0, a: 1.1166666666666671, b: 0.01721420632103996, c: -73.0, d: 18.433333333333334 }
// Fitment Solar Noon: SineFitment { period_mul_2pi: 2.0, a: 0.25, b: 0.03442841264207992, c: 155.0, d: 12.366666666666667 }
// Fitment Day Length: SineFitment { period_mul_2pi: 1.0, a: 2.1166666666666663, b: 0.01721420632103996, c: -84.5, d: 12.150000000000002 }
// 
// Sunrise f(x) = 1.075 * sin(0.01721420632103996 * x + 84.0 * 1.0 * 2 * pi / 365) + 6.341666666666667
// Latex: $f(x) = 1.075 \sin(0.01721420632103996 x + 84.0 \frac{1 \cdot 2 \pi}{365}) + 6.341666666666667$
// 
// Sunset f(x) = 1.116666666666667 * sin(0.01721420632103996 * x - 73.0 * 1.0 * 2 * pi / 365) + 18.433333333333334
// Latex: $f(x) = 1.116666666666667 \sin(0.01721420632103996 x - 73.0 \frac{1 \cdot 2 \pi}{365}) + 18.433333333333334$
// 
// Solar Noon f(x) = 0.25 * sin(0.03442841264207992 * x + 155.0 * 2.0 * 2 * pi / 365) + 12.366666666666667
// 
// Day Length f(x) = 2.1166666666666663 * sin(0.01721420632103996 * x - 84.5 * 1.0 * 2 * pi / 365) + 12.150000000000002


// Seattle, WA
// Sunrise f (x) = 1.383 · sin(0.0172 · x + 102.0·2·π
// ) + 6.583
// 365
// Sunrise A = 1.383, B = 0.0172, C = 102.0, D = 6.583
// Sunset f (x) = 2.408 · sin(0.0172 · x − 95.5·2·π
// 365 ) + 18.775
// Sunset A = 2.408, B = 0.0172, C = −95.5, D = 18.775
// ) + 12.933
// Solar Noon f (x) = 1.05 · sin(0.0344 · x − 186.75·4·π
// 365
// Solar Noon A = 1.05, B = 0.0344, C = −186.75, D = 12.933
// Day Length f (x) = 3.775 · sin(0.0172 · x − 77.5·2·π
// 365 ) + 12.208
// Day Length A = 3.775, B = 0.0172, C = −77.5, D = 12.208
// Valentines:
// 7:16 am ↑ (108°)	5:30 pm ↑ (252°)	10:14:10	+3:12	5:33 am	7:14 pm	6:08 am	6:38 pm	6:44 am	6:02 pm	12:23 pm (29.5°)	

const params= {
	seattle: {
		dayLengthActual: 10 + 14/60 + 10/3600,
		sunriseActual: 7 + 16/60,
		sunsetActual: 17 + 30/60,
		solarNoonActual: 12 + 23/60,
		sunrise: { a: 1.383, b: 0.0172, c: 102.0, d: 6.583 },
		sunset: { a: 2.408, b: 0.0172, c: -95.5, d: 18.775 },
		solarNoon: { a: 1.05, b: 0.0344, c: -186.75, d: 12.933 },
		dayLength: { a: 3.775, b: 0.0172, c: -77.5, d: 12.208 },
	},
	tucson: {
		dayLengthActual: 11 + 1/60 + 17/3600,
		sunriseActual: 7 + 7/60,
		sunsetActual: 18 + 8/60,
		solarNoonActual: 12 + 37/60,
		sunrise: { a: 1.075, b: 0.01721420632103996, c: 84.0, d: 6.341666666666667 },
		sunset: { a: 1.116666666666667, b: 0.01721420632103996, c: -73.0, d: 18.433333333333334 },
		solarNoon: { a: 0.25, b: 0.03442841264207992, c: 155.0, d: 12.366666666666667 },
		dayLength: { a: 2.1166666666666663, b: 0.01721420632103996, c: -84.5, d: 12.150000000000002 },
	}
};

const valentinesDay = new Date(2024, 1, 14);
const valentinesDayDayOfYear = Math.floor((valentinesDay - new Date(valentinesDay.getFullYear(), 0, 0)) / 86400000);

const formatHour = (hour) => {
	return `${Math.floor(hour)}:${Math.floor((hour % 1) * 60).toString().padStart(2, '0')}`;
}

let locationParmas = params[options.location];
if (!locationParmas) {
	throw new Error(`Unknown location ${program.location}`);
}

//const sunrise = (x) => 1.075 * Math.sin(0.01721420632103996 * x + 84.0 * 1.0 * 2 * Math.PI / 365) + 6.341666666666667;
const sunrise = (x) => locationParmas.sunrise.a * Math.sin(locationParmas.sunrise.b * x + locationParmas.sunrise.c * 2 * Math.PI / 365) + locationParmas.sunrise.d;
//const sunset = (x) => 1.116666666666667 * Math.sin(0.01721420632103996 * x - 73.0 * 1.0 * 2 * Math.PI / 365) + 18.433333333333334;
const sunset = (x) => locationParmas.sunset.a * Math.sin(locationParmas.sunset.b * x + locationParmas.sunset.c * 2 * Math.PI / 365) + locationParmas.sunset.d;
//const solarNoon = (x) => 0.25 * Math.sin(0.03442841264207992 * x + 155.0 * 2.0 * 2 * Math.PI / 365) + 12.366666666666667;
const solarNoon = (x) => locationParmas.solarNoon.a * Math.sin(locationParmas.solarNoon.b * x + locationParmas.solarNoon.c * 2 * Math.PI / 365) + locationParmas.solarNoon.d;
//const dayLength = (x) => 2.1166666666666663 * Math.sin(0.01721420632103996 * x - 84.5 * 1.0 * 2 * Math.PI / 365) + 12.150000000000002;
const dayLength = (x) => locationParmas.dayLength.a * Math.sin(locationParmas.dayLength.b * x + locationParmas.dayLength.c * 2 * Math.PI / 365) + locationParmas.dayLength.d;

console.log(`Valentine's day predictions for ` + valentinesDay.getFullYear() + ` is on day ${valentinesDayDayOfYear}`);
console.log(`Sunrise is at ${formatHour(sunrise(valentinesDayDayOfYear))}`);
console.log(`Sunset is at ${formatHour(sunset(valentinesDayDayOfYear))}`);
console.log(`Solar Noon is at ${formatHour(solarNoon(valentinesDayDayOfYear))}`);
console.log(`Day Length lasts ${formatHour(dayLength(valentinesDayDayOfYear))}`);

// actual data
//
const { dayLengthActual, sunriseActual, sunsetActual, solarNoonActual } = locationParmas;

const sunriseDiff = Math.abs(sunriseActual - sunrise(valentinesDayDayOfYear));
const sunsetDiff = Math.abs(sunsetActual - sunset(valentinesDayDayOfYear));
const solarNoonDiff = Math.abs(solarNoonActual - solarNoon(valentinesDayDayOfYear));
const dayLengthDiff = Math.abs(dayLengthActual - dayLength(valentinesDayDayOfYear));

console.log(`Sunrise difference: ${sunriseDiff * 60} minutes`);
console.log(`Sunset difference: ${sunsetDiff * 60} minutes`);
console.log(`Solar Noon difference: ${solarNoonDiff * 60} minutes`);
console.log(`Day Length difference: ${dayLengthDiff * 60} minutes`);

const meanDiff = (sunriseDiff + sunsetDiff + solarNoonDiff + dayLengthDiff) / 4;
console.log(`Mean difference: ${meanDiff * 60} minutes`);

