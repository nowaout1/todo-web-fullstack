import type { Todo } from '$lib/model';

export type TodoDto = {
	id: string;
	title: string;
	created_at: string;
};

export const todoDtoToModel = (dto: TodoDto): Todo => ({
	id: dto.id,
	title: dto.title,
	createdAt: new Date(dto.created_at)
});

export const getTodos = async (
	{ cursor }: { cursor?: string | number | null } = {},
	fetch = window.fetch
) => {
	const apiUrl = new URL('http://localhost:8080/api/v1/todos');

	if (cursor && Number.isInteger(cursor)) {
		apiUrl.searchParams.append('cursor', cursor.toString());
	}

	const { todos, next_cursor } = await fetch(apiUrl, {
		method: 'GET'
	}).then((res) => res.json());

	return {
		todos: todos.map(todoDtoToModel),
		nextCursor: next_cursor
	};
};

export const searchTodos = async (
	{ search, cursor }: { search: string; cursor?: string | number | null },
	fetch = window.fetch
) => {
	const apiUrl = new URL('http://localhost:8080/api/v1/todos/search');

	apiUrl.searchParams.append('search', search);

	if (cursor && Number.isInteger(cursor)) {
		apiUrl.searchParams.append('cursor', cursor.toString());
	}

	const { todos, next_cursor } = await fetch(apiUrl, {
		method: 'GET'
	}).then((res) => res.json());

	return {
		todos: todos.map(todoDtoToModel),
		nextCursor: next_cursor
	};
};
