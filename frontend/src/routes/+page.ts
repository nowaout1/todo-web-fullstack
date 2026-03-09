import { fetchTodos } from '$lib/api';

export const load = async ({ url, fetch }) => {
	const cursor = url.searchParams.get('cursor');

	return fetchTodos({ cursor }, fetch);
};
