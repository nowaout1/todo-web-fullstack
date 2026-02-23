import { todoApi } from '$lib/api';

export const load = async () => {
	const { todos } = await fetch('http://localhost:8080/api/v1/todos?offset=0', {
		method: 'GET'
	}).then((res) => res.json());

	todoApi.fetchTodos();

	return {
		todos
	};
};
