import {Namer} from '@parcel/plugin';
import path from 'path';

export default new Namer({
    name({bundle}) {
        console.log("hi namer!!");
        if (bundle.type === 'png' || bundle.type === 'jpg') {
            let filePath = bundle.getMainEntry().filePath;
            return `images/${path.basename(filePath)}`;
        }

        // Allow the next namer to handle this bundle.
        return null;
    }
});

