<script lang="ts">
	import { onMount } from 'svelte';
	import { enhance } from '$app/forms';
	import { fetchTodos } from '$lib/api';
	import type { Todo } from '$lib/model';
	import type { PageProps } from './$types';
	import type { ActionResult } from '@sveltejs/kit';

	let { data }: PageProps = $props();
	const updateData = (patch: Partial<typeof data>) => (data = { ...data, ...patch });

	const todos = $derived(data.todos);
	const nextCursor = $derived(data.nextCursor);

	const prependTodos = (...todos: Todo[]) => updateData({ todos: [...todos, ...data.todos] });
	const appendTodos = (...todos: Todo[]) => updateData({ todos: [...data.todos, ...todos] });
	const setCursor = (cursor: number) => updateData({ nextCursor: cursor });

	let query = $state('');
	let isSubmitting = $state(false);

	const isQueryEmpty = $derived(query.length === 0);

	onMount(() => {
		const intersectionHandler = async (entries: IntersectionObserverEntry[]) => {
			for (const entry of entries) {
				if (entry.isIntersecting && nextCursor) {
					const { todos: newTodos, nextCursor: cursor } = await fetchTodos({
						cursor: nextCursor
					});

					appendTodos(...newTodos);
					setCursor(cursor);
				}
			}
		};

		const observer = new IntersectionObserver(intersectionHandler, {
			threshold: 1.0
		});

		observer.observe(requestNextElement);

		return () => {
			observer.disconnect();
		};
	});

	let requestNextElement: HTMLDivElement;
</script>

<section>
	<input
		type="search"
		placeholder="Search"
		class="w-full border"
		oninput={(event) => {
			query = (event.target as HTMLInputElement)?.value;
		}}
	/>

	<div class="flex max-h-80 w-120 flex-col overflow-auto">
		{#if isQueryEmpty}
			{@render todoList(todos)}
		{:else}
			Nothing to view...
		{/if}
		<div class="min-h-2 w-full" bind:this={requestNextElement}></div>
	</div>

	<form
		method="post"
		action="?/create"
		class="grid w-full grid-cols-[1fr_auto]"
		use:enhance={() => {
			isSubmitting = true;

			return async ({ result }: { result: ActionResult }) => {
				if (result.type === 'success') {
					const todo = result.data?.todo;

					if (todo) {
						prependTodos(todo);
					}

					isSubmitting = false;
				}
			};
		}}
	>
		<input name="title" type="text" placeholder="Title" class="w-full border" />
		<button type="submit" class="border" disabled={isSubmitting}>Add todo</button>
	</form>
</section>

{#snippet todoList(todos: Todo[])}
	{#each todos as { id, title, createdAt } (id)}
		{@const date = new Intl.DateTimeFormat([], {
			dateStyle: 'short',
			timeStyle: 'short'
		}).format(createdAt)}

		<div class="grid grid-cols-[1fr_auto]">
			<span>{title}</span>
			<span>{date}</span>
		</div>
	{/each}
{/snippet}
