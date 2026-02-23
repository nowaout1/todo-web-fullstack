import { createApi } from '$lib/utils/api-utils';

export type TodoDto = {
	id: string;
	title: string;
	created_at: string;
};

const API_URL = import.meta.env.VITE_API_URL;

if (API_URL == null) {
	throw new Error('Todo api error: `API_URL` not specified');
}

const request = createApi(API_URL);

export const fetchTodos = async () => {
	const response = await request('/todos', { offset: 0 }, { method: 'GET' }).then((res) =>
		res.json()
	);
	console.log(response);
};

export const fetchTodoById = async () => {
	request('/todos', { id: '123' }, { method: 'GET' });
};

export const createTodo = async (title: string) => {
	request('/todos', {}, { method: 'POST' });
};

export const deleteTodoById = async (id: string) => {
	request('/todos', {}, { method: 'DELETE' });
};
