import { todoDtoToModel, type TodoDto } from '$lib/api';

export const actions = {
	create: async ({ fetch, request }) => {
		const formData = await request.formData();
		const title = formData.get('title');
		const apiUrl = new URL('http://localhost:8080/api/v1/todos');

		const { todo }: { todo: TodoDto | null } = await fetch(apiUrl, {
			method: 'POST',
			body: JSON.stringify({ title }),
			headers: {
				'Content-Type': 'application/json'
			}
		}).then((res) => res.json());

		return { todo: todo ? todoDtoToModel(todo) : null };
	}
};
