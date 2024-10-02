/** @type {import('../../../.svelte-kit/types/src/routes').PageLoad} */

import {redirect} from "@sveltejs/kit";

export const actions = {
    logout: async ({cookies, request}) => {
        cookies.delete('token', { path: '/' });
        return redirect(303, '/')
    }
}
