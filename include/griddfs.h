/* From https://youtu.be/zmtHaZG7pPc?t=22m19s */
struct griddfs_error {
	const char *message;
	int failed;
	int code;
};

struct griddfs_matrix {
	unsigned char *data;
	unsigned rows;
	unsigned cols;
};

struct griddfs_rect {
	unsigned top;
	unsigned left;
	unsigned width;
	unsigned height;
};

void griddfs_init(void);

void griddfs_dfs(struct griddfs_matrix * dirs,
		 const struct griddfs_rect * sources,
		 struct griddfs_matrix * marks,
		 unsigned char mark,
		 struct griddfs_error *);

void griddfs_free(char *);
