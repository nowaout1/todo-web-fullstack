export const createApi =
	(apiUrl: string) =>
	async (path: string, params?: Record<string, string | number | boolean>, init?: RequestInit) => {
		const url = new URL(`${apiUrl}${path}`);

		if (params) {
			Object.entries(params).forEach(([key, value]) => {
				url.searchParams.append(key, value.toString());
			});
		}

		return fetch(url.toString(), init);
	};
