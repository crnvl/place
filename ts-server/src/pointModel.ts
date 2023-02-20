import { model, Schema } from 'mongoose';

const pointSchema = new Schema({
    x: Number,
    y: Number,
    color: Number,
});

export const Point = model('Point', pointSchema);