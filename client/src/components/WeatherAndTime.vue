<template>
    <div class="weather-time-widget text-xl">
        <div v-if="error" class="error">{{ error }}</div>
        <div v-else-if="useWeatherAPI">
            <div class="weather-widget">
                <h3>Weather in {{ location }}</h3>
                <p>{{ weather.description }}</p>
                <p>Temperature: {{ weather.temperature }}°C</p>
            </div>
            <div class="time-widget">
                <h3>{{ location }}</h3>
                <p>{{ localTime }}</p>
            </div>
        </div>
        <div v-else>
            <div class="time-widget">
                <h3>{{ location }} {{ localTime }}</h3>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import axios from "axios";
import { onMounted, ref, watch } from "vue";

interface WeatherData {
	description: string;
	temperature: number;
}

const props = defineProps<{
	location: string;
}>();

const validLocations = ["Las Vegas", "North Bay"];
const weather = ref<WeatherData>({ description: "", temperature: 0 });
const localTime = ref("");
const error = ref("");
const useWeatherAPI = false;

const fetchWeatherData = async (location: string) => {
	try {
		const response = await axios.get(
			`https://api.weatherapi.com/v1/current.json?key=YOUR_API_KEY&q=${location}`,
		);
		const data = response.data;
		weather.value = {
			description: data.current.condition.text,
			temperature: data.current.temp_c,
		};
	} catch (err) {
		error.value = "Failed to fetch weather data";
	}
};

const getOfficeTime = () => {
	const timeFormatter = new Intl.DateTimeFormat("en-US", {
		hour: "numeric",
		minute: "numeric",
		hour12: true,
		timeZone: "America/Los_Angeles", // Las Vegas timezone
	});

	const northBayFormatter = new Intl.DateTimeFormat("en-US", {
		hour: "numeric",
		minute: "numeric",
		hour12: true,
		timeZone: "America/Toronto", // North Bay timezone
	});

	const now = new Date();

	if (props.location === "Las Vegas")
		localTime.value = timeFormatter.format(now);
	else localTime.value = northBayFormatter.format(now);

	return {
		lasVegas: timeFormatter.format(now),
		northBay: northBayFormatter.format(now),
	};
};

watch(
	() => props.location,
	(newLocation) => {
		if (validLocations.includes(newLocation)) {
			if (useWeatherAPI) fetchWeatherData(newLocation);
			getOfficeTime();
		} else {
			error.value = "Invalid location";
		}
	},
);

onMounted(() => {
	if (validLocations.includes(props.location)) {
		if (useWeatherAPI) fetchWeatherData(props.location);
		getOfficeTime();
		setInterval(getOfficeTime, 1000); // Update time every second
	} else {
		error.value = "Invalid location";
	}
});
</script>

<style scoped>
    .weather-widget,
    .time-widget {
        padding: 1rem;
        width: 400px;
        text-align: center;
    }

    .error {
        color: red;
    }
</style>