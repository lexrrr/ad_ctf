/** @type {import('../../../.svelte-kit/types/src/routes').PageServerLoad} */
import { env } from '$env/dynamic/public';
import {error, redirect} from "@sveltejs/kit";

export async function load({cookies, url}) {
    let ipSets = [];
    const token = cookies.get('token')

    const response = await fetch(env.PUBLIC_BACKEND_URL + '/ip-set', {
        method: 'GET',
        headers: {
            'Content-Type': 'application/json',
            'Authorization': `Bearer ${token}`
        },
    })
        .then(response => response.json())
        .then(json => {
            ipSets = json;
        })
        .catch(error => console.log('Error:', error))
    return { 'ipSets': ipSets }
}

export const actions = {
    create: async ({ cookies, request }) => {
        const data = await request.formData();
        const name = data.get('name');
        const description = data.get('description');
        const ips = data.get('ips');
        const token = cookies.get('token')

        const response = await fetch(env.PUBLIC_BACKEND_URL + '/ip-set', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                'Authorization': `Bearer ${token}`
            },
            body: JSON.stringify({"name": name, "description": description, "ips": ips.split(";")})
        });

        if (response.ok) {
            let ipSets = [];

            const response = await fetch(env.PUBLIC_BACKEND_URL + '/ip-set', {
                method: 'GET',
                headers: {
                    'Content-Type': 'application/json',
                    'Authorization': `Bearer ${token}`
                },
            })
                .then(response => response.json())
                .then(json => {
                    ipSets = json;
                })
                .catch(error => console.log('Error:', error))
            return { 'ipSets': ipSets }
        } else {
            console.error('Create Request failed');
        }
        return redirect(303, '/ip-set')
    },
};