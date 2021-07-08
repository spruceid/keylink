<script context="module" lang="ts">
	import type { Load } from '@sveltejs/kit';

	export const load: Load = async ({ page }) => {
		return { props: { page_name: page.path.slice(1) } };
	};
</script>

<script lang="ts">
	import { fade } from 'svelte/transition';
	import { AppBar, Spacer, Tooltip, Button } from 'smelte';
	import logo from '../../static/favicon.svg';

	import 'smelte/src/tailwind.css';

	import dark from 'smelte/src/dark';
	const darkMode = dark();

	export let page_name: string;
</script>

<svelte:head>
	<title>
		{page_name ? `${page_name} |` : ''} Keylink
	</title>
	<meta name="description" content="Keylink. {page_name}" />
</svelte:head>

<AppBar class={(i) => i.replace('primary-300', 'dark-600')}>
	<a href="/" class="px-2 md:px-8 flex items-center">
		<img src={logo} alt="Keylink logo" width="44" />
		<h6 class="pl-3 text-white tracking-widest font-thin text-lg">KEYLINK</h6>
	</a>
	<Spacer />
	<Tooltip>
		<!-- // I think it doesn't work because of import 'svelte/store' -->
		<span slot="activator">
			<Button
				bind:value={$darkMode}
				icon="wb_sunny"
				small
				flat
				remove="p-1 h-4 w-4"
				iconClass="text-white"
				text
			/>
		</span>
		{$darkMode ? 'Disable' : 'Enable'} dark mode
	</Tooltip>
</AppBar>

<main
	class="relative p-8 lg:max-w-3xl mx-auto mb-10 mt-24 md:ml-64 md:pl-16 md:max-w-md md:px-3"
	transition:fade={{ duration: 300 }}
>
	<slot />
</main>
