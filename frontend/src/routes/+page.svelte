<script lang="ts">
	import { onMount } from 'svelte';
	import { enhance } from '$app/forms';
	import { getTodos, todoDtoToModel } from '$lib/api';
	import type { Todo } from '$lib/model';
	import type { PageProps } from './$types';
	import type { ActionResult } from '@sveltejs/kit';

	let { data }: PageProps = $props();

	let todos = $derived(data.todos);
	let nextCursor = $derived(data.nextCursor);

	const deleteTodo = (id: Todo['id']) => (todos = todos.filter((todo: Todo) => todo.id != id));
	const prependTodos = (...newTodos: Todo[]) => (todos = [...newTodos, ...todos]);
	const appendTodos = (...newTodos: Todo[]) => (todos = [...todos, ...newTodos]);

	let search = $state('');
	const isSearchEmpty = $derived(search.length === 0);

	let isSubmitDisabled = $state(false);

	const getNextTodos = async () => {
		const { todos: newTodos, nextCursor: cursor } = await getTodos({
			cursor: nextCursor
		});

		appendTodos(...newTodos);
		nextCursor = cursor;
	};

	onMount(() => {
		const intersectionHandler = async (entries: IntersectionObserverEntry[]) => {
			for (const entry of entries) {
				if (entry.isIntersecting && nextCursor) {
					getNextTodos();
				}
			}
		};

		const observer = new IntersectionObserver(intersectionHandler, {
			threshold: 1.0
		});

		observer.observe(observableElement);

		return () => {
			observer.disconnect();
		};
	});

	let observableElement: HTMLDivElement;
</script>

<section>
	<input
		type="search"
		placeholder="Search"
		class="w-full border"
		oninput={(event) => {
			search = (event.target as HTMLInputElement)?.value;
		}}
	/>

	<div class="flex max-h-80 w-120 flex-col overflow-auto">
		{#if isSearchEmpty}
			{@render todoList(todos)}
		{:else}
			*there should be todos here*
		{/if}
		<div class="min-h-2 w-full" bind:this={observableElement}></div>
	</div>

	<form
		method="post"
		action="?/create"
		class="grid w-full grid-cols-[1fr_auto]"
		use:enhance={() => {
			isSubmitDisabled = true;

			return async ({ result }: { result: ActionResult }) => {
				if (result.type === 'success') {
					const dto = result.data?.todo;

					if (dto) {
						const todo = todoDtoToModel(dto);
						prependTodos(todo);
					}
				}

				isSubmitDisabled = false;
			};
		}}
	>
		<input name="title" type="text" placeholder="Title" class="w-full border" />
		<button type="submit" class="border" disabled={isSubmitDisabled}>Add todo</button>
	</form>
</section>

{#snippet todoList(todos: Todo[])}
	{#if todos.length > 0}
		{#each todos as { id, title, createdAt } (id)}
			{@const date = new Intl.DateTimeFormat([], {
				dateStyle: 'short',
				timeStyle: 'short'
			}).format(createdAt)}

			<div class="grid grid-cols-[1fr_auto_auto] gap-2">
				<span>{title}</span>
				<span>{date}</span>
				<form
					method="POST"
					action="?/delete"
					use:enhance={() => {
						isSubmitDisabled = true;

						return async ({ result }: { result: ActionResult }) => {
							if (result.type === 'success') {
								deleteTodo(id);
							}

							isSubmitDisabled = false;
						};
					}}
				>
					<input type="hidden" name="id" value={id} />
					<button type="submit" class="border" disabled={isSubmitDisabled}>done</button>
				</form>
			</div>
		{/each}
	{:else}
		<span>Nothing to view...</span>
	{/if}
{/snippet}
