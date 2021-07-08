<script lang="ts">
	import { page } from '$app/stores';
	import { username, email } from '$lib/stores';
	import { List, ListItem, NavigationDrawer, Icon } from 'smelte';

	const menu = [{ to: '/dashboard', text: 'Dashboard' }];
</script>

<NavigationDrawer persistent elevation>
	<Icon>account_circle</Icon>{$username ? $username : $email}
	<List items={menu}>
		<span slot="item" let:item class="cursor-pointer">
			<a href={item.to}>
				<ListItem
					selected={$page.path.includes(item.to)}
					{...item}
					dense
					selectedClasses="bg-primary-transDark dark:bg-primary-transLight hover:bg-primary-transDark dark-hover:bg-primary-transLight"
				/>
			</a>
		</span>
	</List>
	<hr />
</NavigationDrawer>

<slot />
