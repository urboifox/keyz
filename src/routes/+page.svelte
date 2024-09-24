<script lang="ts">
	import { KEYS, SPECIAL_KEYS, SPECIAL_KEYS_MAP } from '$lib';
	import { listen, type Event } from '@tauri-apps/api/event';
	import { onMount } from 'svelte';

	let strokes: string[] = [];
	const special_keys = ['Shift', 'Control', 'Alt', 'Meta'] as const;
	let pressed_keys: string[] = [];

	onMount(() => {
		let interval: any;

		// listen for key down events
		const unlistenKeydown = listen('key-down', async (event: Event<{ key: string }>) => {
			// clear the strokes after 3 seconds
			clearInterval(interval);
			interval = setInterval(() => {
				strokes = [];
			}, 1200);

			const pressed_key = event.payload.key;
			if (Object.keys(SPECIAL_KEYS).includes(pressed_key)) {
				strokes = [];
				pressed_keys = [
					...pressed_keys,
					SPECIAL_KEYS_MAP[pressed_key as keyof typeof SPECIAL_KEYS_MAP]
				];
			} else {
				const key_display = KEYS[pressed_key as keyof typeof KEYS] || '';
				strokes = [...strokes, key_display].slice(-6)
			}
		});

		return async () => {
			unlistenKeydown.then((unlisten) => unlisten());
			clearInterval(interval);
		};
	});

	onMount(() => {
		// listen for key up events
		const unlistenKeyup = listen('key-up', async (event: Event<{ key: string }>) => {
			const pressed_key = event.payload.key;

			if (Object.keys(SPECIAL_KEYS).includes(pressed_key)) {
				pressed_keys = pressed_keys.filter(
					(key) => key !== SPECIAL_KEYS_MAP[pressed_key as keyof typeof SPECIAL_KEYS_MAP]
				);
			}
		});

		return async () => {
			unlistenKeyup.then((unlisten) => unlisten());
		};
	});
</script>

<main>
	<div class="overlay" data-tauri-drag-region />
	<div class="keys">
		{#each strokes as stroke, i (i)}
			<span class="key">{stroke}</span>
		{/each}
	</div>

	<div class="special-keys">
		{#each special_keys as key, i (i)}
			<span class:active={pressed_keys.includes(key)} class="key">{SPECIAL_KEYS[key]}</span>
		{/each}
	</div>
</main>

<style>
	main {
		display: flex;
		align-items: center;
		justify-content: center;
		flex-direction: column;
		width: 100%;
		padding: 1rem;
		height: 100vh;
		gap: 0.5rem;
		position: relative;
	}

	.overlay {
		position: absolute;
		inset: 0;
		width: 100%;
		height: 100%;
		z-index: 10;
	}

	.keys {
		display: flex;
		align-items: center;
		justify-content: center;
		flex-grow: 1;
		width: 100%;
		overflow-x: hidden;
		max-width: 100%;
		font-size: 2rem;
	}

	.special-keys {
		display: flex;
		border-top: 1px solid #ffffff20;
		padding-top: 0.7rem;
		align-items: center;
		width: 100%;
		justify-content: space-evenly;
		gap: 0.2rem;
	}

	.special-keys .key {
		font-size: 1.3rem;
		opacity: 0.5;
		transition: opacity 20ms;
	}

	.special-keys .key.active {
		opacity: 1;
	}
</style>
