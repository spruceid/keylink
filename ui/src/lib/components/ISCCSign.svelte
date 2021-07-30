<script lang="ts">
	import { Dialog, Button, TextField } from 'smelte';

	import Highlight from 'svelte-highlight';
	import json from 'svelte-highlight/src/languages/json';
	import github from 'svelte-highlight/src/styles/github';
	let code = '';

	let showDialog1 = false;
	let showDialog2 = false;
	export let keyName;
	let files;

	let submit = async () => {
		const formData = new FormData();
		formData.append('key', keyName);
		formData.append('doc', files[0]);
		let res = await fetch('/iscc/issue', {
			method: 'POST',
			body: formData
		});
		if (res.ok) {
			code = JSON.stringify(await res.json(), null, 2);
			showDialog2 = true;
			showDialog1 = false;
		} else {
			throw new Error(await res.text());
		}
	};
</script>

<svelte:head>
	{@html github}
</svelte:head>

<Dialog bind:value={showDialog1}>
	<h5 slot="title">Issue ISCC VC</h5>
	<h6>File</h6>
	<input type="file" bind:files />
	<blockquote class="pl-2 mt-2 mb-10 border-l-8 border-primary-300">
		<p>Only .txt files are supported for now.</p>
	</blockquote>
	<div slot="actions">
		<Button text on:click={() => (showDialog1 = false)}>Cancel</Button>
		<Button text outlined disabled={!files} on:click={async () => await submit()}>Issue</Button>
	</div>
</Dialog>

<Dialog bind:value={showDialog2}>
	<h5 slot="title">VC</h5>
	<Highlight language={json} {code} />
	<div slot="actions">
		<Button text on:click={() => (showDialog2 = false)}>OK</Button>
	</div>
</Dialog>

<!-- <div class="py-2"> -->
<Button on:click={() => (showDialog1 = true)}>Issue ISCC VC</Button>
<!-- </div> -->
