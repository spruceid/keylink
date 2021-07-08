<script lang="ts">
	import { Button, Dialog } from 'smelte';
	import { keys, logged_in, redirect, credential } from '$lib/stores';
	import { onMount, setContext, getContext } from 'svelte';
	import { goto, prefetch } from '$app/navigation';
	// import { page } from '$app/stores';
	import NewKey from '$lib/components/NewKey.svelte';
	import Highlight from 'svelte-highlight';
	import json from 'svelte-highlight/src/languages/json';
	import github from 'svelte-highlight/src/styles/github';

	let confirmationDialog = false;
	let signedCredential = '';

	onMount(async () => {
		// First request
		// const redirect_ = getContext<string>('redirect');
		if ($redirect == null) {
			// if (!redirect_) {
			// TODO workaround for https://github.com/sveltejs/kit/issues/669
			let query = new URLSearchParams(document.location.search);
			// redirect_url = $page.query.get('url');
			let redirect_url = query.get('url');
			// Bad request, we open the app regularly
			if (redirect_url == null) {
				goto('/');
			}
			if (!redirect_url.startsWith('http')) {
				redirect_url = `http://${redirect_url}`;
			}
			// TODO this should be in the headers or cookies, not sure how to do it ATM
			// credential = $page.query.get('credential');
			let credential_query = query.get('credential');
			$redirect = redirect_url;
			// setContext<string>('redirect', $redirect);
			$credential = credential_query;
		}

		// TODO can't get it to work, I think it drops all the state when you go up in the hierarchy of routes
		// if (!$logged_in) {
		// 	goto('/');
		// }

		const res = await fetch('/keys');
		if (res.ok) {
			keys.set(await res.json());
		} else {
			const res = await fetch('/user');
			if (!res.ok) {
				// TODO that's not a great way to manage the session
				logged_in.set(false);
				goto('/');
			}
			const { message } = await res.json();
			throw new Error(message);
		}
	});

	let sign = async (keyName: string) => {
		let res = await fetch('/credentials/issue', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({
				key: keyName,
				credential: JSON.parse($credential)
			})
		});
		if (res.ok) {
			signedCredential = JSON.stringify(await res.json(), null, 2);
			confirmationDialog = true;
		} else {
			throw new Error(await res.text());
		}
	};

	let redirectBack = async () => {
		// TODO handle URLs with query parameters
		// TODO put signed credential in headers
		goto(`${$redirect}?credential=${JSON.stringify(signedCredential)}`);
	};
</script>

<svelte:head>
	{@html github}
</svelte:head>

<h2>Request for Signature</h2>

<h3>Credential</h3>
{#if credential}
	<Highlight language={json} code={JSON.stringify(JSON.parse($credential), null, 2)} />
{/if}

<h3>Choose Key</h3>
{#each $keys as key}
	<div class="py-2">
		<Button on:click={() => sign(key)}>{key}</Button>
	</div>
{/each}
<NewKey />

<Dialog bind:value={confirmationDialog}>
	<h5 slot="title">Signed Credential</h5>
	<Highlight language={json} code={signedCredential} />
	<div slot="actions">
		<Button text on:click={() => redirectBack()}>Confirm</Button>
	</div>
</Dialog>
