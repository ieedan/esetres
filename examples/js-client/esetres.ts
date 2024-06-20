export type CreateOptions = {
	/** The esetres access token taken from the server
	 *
	 *  [How to get a token](https://github.com/ieedan/esetres?tab=readme-ov-file#tokens)
	 */
	token: string;
	/** Ex: http://127.0.0.1:3000 */
	host: string;
};

export type Scope = "public" | "private";

export type GetOptions = {
	/** Bucket to fetch from. */
	bucket: string;
	/** The scope to fetch from. */
	scope: Scope;
	/** The name of the file including the extension that you want to fetch
	 *
	 * @example 'something.txt'
	 */
	fileName: string;
};

export type UploadOptions = {
	/** Bucket to upload to.
	 *
	 *  [How to create a bucket](https://github.com/ieedan/esetres?tab=readme-ov-file#buckets)
	 */
	bucket: string;
	/** The scope to upload to. */
	scope: Scope;
	/** The name of the file including the extension that you want to fetch
	 *
	 * @example 'something.txt'
	 */
	fileName: string;
	/** The file content as an array buffer 
     * 
     * The result of: 
     *  ```ts
     *  const reader = new FileReader();

        reader.onload = async (e) => {
            const buffer = e.target.result as ArrayBuffer;
        }

        reader.readAsArrayBuffer(file);
     *  ```
     */
	buffer: ArrayBuffer;
};

/** Initializes a new client.
 *
 *  This should be called from the SERVER SIDE ONLY!
 *  Requests made with this client will contain the token in the authorization header and
 *  will allow anyone on the client side to see the token.
 */
export function createClient({ host, token }: CreateOptions) {
	// Trims trailing "/"
	if (host.endsWith("/")) {
		host = host.slice(0, host.length - 1);
	}
	return {
		get: async ({ bucket, scope, fileName }: GetOptions): Promise<ArrayBuffer> => {
			const headers = new Headers();
			headers.append("Authorization", `Bearer ${token}`);

			const init = {
				method: "GET",
				headers,
			};

			const res = await fetch(`${host}/buckets/${bucket}/${scope}/${fileName}`, init);

			return await res.arrayBuffer();
		},
		upload: async ({ bucket, scope, fileName, buffer }: UploadOptions): Promise<string | null> => {
			const bytes = new Uint8Array(buffer);

			const headers = new Headers();
			headers.append("Authorization", `Bearer ${token}`);

			const init = {
				method: "PUT",
				body: bytes,
				headers,
			};

			const { ok, body } = await fetch(`${host}/buckets/${bucket}/${scope}/${fileName}`, init)
				.then((response) => {
					return Promise.all([response.ok, response.text().catch(() => null)]);
				})
				.then((a) => {
					return { ok: a[0], body: a[1] };
				});

            return ok ? body : null;
		},
	};
}
