import type { RequestEvent } from './$types'

export const actions = {
	default: async ({ request }: RequestEvent) => {
		const data = await request.formData()
		console.log(data)
	},
}
