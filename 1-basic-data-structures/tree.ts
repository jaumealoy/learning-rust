class TreeNode {
    public value: number;
    public left: TreeNode;
    public right: TreeNode;

    constructor(value: number) {
        this.value = value;
    }
}

class Tree {
    private root: TreeNode;

    constructor() {
        this.root = null;
    }

    public add(value: number) : void {
        let parent: TreeNode = null;
        let current = this.root;
        
        let isLeftChild: boolean = false;
        while (current != null) {
            parent = current;
            if (value > current.value) {
                current = current.right;
                isLeftChild = false;
            } else {
                current = current.left;
                isLeftChild = true;
            }
        }

        if (parent == null) {
            this.root = new TreeNode(value);
        } else {
            if (isLeftChild) {
                parent.left = new TreeNode(value);
            } else{
                parent.right = new TreeNode(value);
            }
        }
    }
}