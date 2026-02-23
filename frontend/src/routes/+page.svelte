<script lang="ts">
	import { page } from '$app/state';

	const { todos } = page.data;
</script>

{#each todos as todo (todo.id)}
	<span>{todo.title}</span>
{/each}
