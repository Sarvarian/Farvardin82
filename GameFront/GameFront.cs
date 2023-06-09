﻿using Microsoft.Xna.Framework;
using Microsoft.Xna.Framework.Graphics;
using Microsoft.Xna.Framework.Input;

namespace GameFront;

public class GameFront : Game
{
	private GraphicsDeviceManager graphics_;
	private SpriteBatch spriteBatch_;
	private GameCore.Game game_ = new();

	public GameFront()
	{
		graphics_ = new GraphicsDeviceManager(this);
		Content.RootDirectory = "Content";
		IsMouseVisible = true;
	}

	protected override void Initialize()
	{
		// TODO: Add your initialization logic here

		base.Initialize();
	}

	protected override void LoadContent()
	{
		spriteBatch_ = new SpriteBatch(GraphicsDevice);

		// TODO: use this.Content to load your game content here
	}

	protected override void Update(GameTime gameTime)
	{
		if (GamePad.GetState(PlayerIndex.One).Buttons.Back == ButtonState.Pressed ||
			Keyboard.GetState().IsKeyDown(Keys.Escape))
			Exit();

		// TODO: Add your update logic here

		base.Update(gameTime);
	}

	protected override void Draw(GameTime gameTime)
	{
		GraphicsDevice.Clear(Color.CornflowerBlue);

		// TODO: Add your drawing code here

		base.Draw(gameTime);
	}
}