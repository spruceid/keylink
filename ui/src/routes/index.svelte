<script lang="ts">
	import { username, email, logged_in, redirect } from '$lib/stores';
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';

	// Can't get cookies to work for the life of me
	onMount(async () => {
		const res = await fetch('/user');
		if (res.ok) {
			const user = await res.json();
			username.set(user.name);
			email.set(user.email);
			logged_in.set(true);
			if ($redirect != null) {
				goto('/redirect');
			} else {
				goto('/dashboard');
			}
		} else {
			logged_in.set(false);
			goto('/signin');
		}
	});
</script>
