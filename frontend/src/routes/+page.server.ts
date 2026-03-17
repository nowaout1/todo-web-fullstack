import { fail } from '@sveltejs/kit';

export const actions = {
	create: async ({ fetch, request }) => {
		const formData = await request.formData();
		const title = formData.get('title');
		const apiUrl = 'http://localhost:8080/api/v1/todos';

		const res = await fetch(apiUrl, {
			method: 'POST',
			body: JSON.stringify({ title }),
			headers: {
				'Content-Type': 'application/json'
			}
		});

		const data = await res.json();

		return { ...data, status: res.status };
	},
	delete: async ({ fetch, request }) => {
		const formData = await request.formData();
		const id = formData.get('id');

		if (!id) {
			return fail(400, { msg: 'Todo id not specified' });
		}

		const apiUrl = `http://localhost:8080/api/v1/todos/${id}`;

		const res = await fetch(apiUrl, {
			method: 'DELETE'
		});

		const data = await res.json();

		return { ...data, status: res.status };
	}
};
