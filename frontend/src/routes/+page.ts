import { getTodos, searchTodos } from '$lib/api';

export const load = async ({ url, fetch }) => {
	const cursor = url.searchParams.get('cursor');
	const search = url.searchParams.get('search');

	const isSearching = search && search.length > 0;

	if (isSearching) {
		return searchTodos({ search, cursor }, fetch);
	}

	return getTodos({ cursor }, fetch);
};
